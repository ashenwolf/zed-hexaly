//! Launches `hexaly-lsp` for Hexaly files.
//!
//! The extension does as little as possible on purpose. Everything about the language — parsing,
//! diagnostics, completion — is in the language server, which is a plain executable speaking LSP
//! over stdio and works the same in Neovim, Helix and Emacs. This file only answers "where is that
//! binary", which is genuinely the editor's question to answer.
//!
//! Note what is deliberately *not* here: locating the Hexaly installation. Zed offers
//! `worktree.which()`, which would work and would also strand every other editor, so the server
//! discovers Hexaly itself.

use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId, LanguageServerInstallationStatus, Os, Result,
    process::Command, settings::LspSettings,
};

const SERVER: &str = "hexaly-lsp";
const REPOSITORY: &str = "ashenwolf/hexaly-lsp";

/// The server release this extension is built against.
///
/// Pinned rather than "latest" on purpose. The extension, the tree-sitter grammar `rev` and the
/// server are three coupled versions; resolving the newest release at runtime would let a server
/// change reach users of an older extension, which is a class of bug that only reproduces on someone
/// else's machine. Bumping this is a deliberate commit, the same as bumping the grammar.
const SERVER_VERSION: &str = "v0.2.0";

struct HexalyExtension {
    /// Path to a binary this process already downloaded, if it did.
    ///
    /// Caching the answer in memory is what stops a second worktree, or a reconnect, from
    /// re-downloading. A cold start still re-checks the release, which is one API call and correct:
    /// it is also how a partially-extracted download from a previous run gets repaired.
    downloaded: Option<String>,
}

impl zed::Extension for HexalyExtension {
    fn new() -> Self {
        Self { downloaded: None }
    }

    fn language_server_command(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // An explicit setting wins, so a user can point at a build they are working on:
        //
        //     "lsp": { "hexaly-lsp": { "binary": { "path": "~/Work/hexaly-lsp/target/release/hexaly-lsp" } } }
        let settings = LspSettings::for_worktree(SERVER, worktree).ok();
        let configured = settings.as_ref().and_then(|settings| settings.binary.as_ref());

        if let Some(path) = configured.and_then(|binary| binary.path.clone()) {
            return Ok(zed::Command {
                command: path,
                args: configured
                    .and_then(|binary| binary.arguments.clone())
                    .unwrap_or_default(),
                env: worktree.shell_env(),
            });
        }

        // Then a binary the user installed themselves, found on `$PATH` - the documented convention
        // for extensions wrapping a server, and what someone developing the server expects to win
        // over a downloaded copy.
        if let Some(command) = worktree.which(SERVER) {
            return Ok(zed::Command {
                command,
                args: Vec::new(),
                env: worktree.shell_env(),
            });
        }

        // Then the location `cargo install` uses.
        //
        // This is not redundant with `which`. A GUI-launched Zed inherits its `$PATH` from
        // /etc/paths, which does not include ~/.cargo/bin - that is added by a shell profile and so
        // only reaches processes started from a shell. The result is that `cargo install hexaly-lsp`
        // succeeds, the binary works in a terminal, and the extension still cannot find it.
        //
        // Unlike the two branches above, this path is a guess, so it is *verified* by running it:
        // there is no filesystem API to test existence with, but a binary that answers `--version`
        // is present and executable, which is the whole question. Guessing without checking would
        // shadow the download below for exactly the users who need it - anyone who never ran
        // `cargo install`.
        if let Some(command) = cargo_install_location(worktree).filter(|path| responds_to_version(path)) {
            return Ok(zed::Command {
                command,
                args: Vec::new(),
                env: worktree.shell_env(),
            });
        }

        // Finally, fetch a released binary. This is what makes the extension work for someone who
        // has never installed Rust, which was the whole problem with `cargo install` as the only
        // channel.
        Ok(zed::Command {
            command: self.download(server_id)?,
            args: Vec::new(),
            env: worktree.shell_env(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        // Forwarded verbatim: the server reads `hexalyPath` from here, and every other client
        // passes the same key through its own `init_options`. Keeping the shape identical across
        // editors is what stops the server growing Zed-specific configuration.
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|settings| settings.initialization_options))
    }
}

impl HexalyExtension {
    /// The pinned release's binary for this platform, downloading it if this process has not already.
    fn download(&mut self, server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.downloaded {
            return Ok(path.clone());
        }

        zed::set_language_server_installation_status(server_id, &LanguageServerInstallationStatus::CheckingForUpdate);

        let release = zed::github_release_by_tag_name(REPOSITORY, SERVER_VERSION)?;
        let asset_name = asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("{SERVER_VERSION} has no asset {asset_name}; this platform is not released"))?;

        // Versioned directory, so a later pin downloads afresh instead of trusting whatever an
        // earlier one left behind - and so the previous binary stays usable if a download fails.
        let directory = format!("{SERVER}-{SERVER_VERSION}");
        let binary = format!("{directory}/{SERVER}{}", executable_suffix()?);

        zed::set_language_server_installation_status(server_id, &LanguageServerInstallationStatus::Downloading);
        zed::download_file(&asset.download_url, &directory, archive_type()?)?;
        // Zed unpacks the archive but does not restore permissions, so this is required rather than
        // defensive: without it the spawn fails with a permission error.
        zed::make_file_executable(&binary)?;

        zed::set_language_server_installation_status(server_id, &LanguageServerInstallationStatus::None);
        self.downloaded = Some(binary.clone());
        Ok(binary)
    }
}

zed::register_extension!(HexalyExtension);

/// The release asset for the current platform, named as the release workflow names it.
///
/// The names are the Rust target triples the workflow builds, which keeps this table and the CI
/// matrix legible against each other - a platform missing here is a platform not built there.
fn asset_name() -> Result<String> {
    let (os, architecture) = zed::current_platform();
    let triple = match (os, architecture) {
        (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
        (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
        (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
        (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
        (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
        // 32-bit x86, and Windows on ARM, are not built. Saying so beats letting the lookup fail
        // with a missing-asset message that reads like a broken release.
        _ => return Err(format!("{SERVER} has no release build for this platform")),
    };

    let extension = if matches!(os, Os::Windows) { "zip" } else { "tar.gz" };
    Ok(format!("{SERVER}-{triple}.{extension}"))
}

/// How Zed should unpack the asset, matching the archive the workflow produced for this platform.
fn archive_type() -> Result<DownloadedFileType> {
    match zed::current_platform().0 {
        Os::Windows => Ok(DownloadedFileType::Zip),
        _ => Ok(DownloadedFileType::GzipTar),
    }
}

/// The executable suffix for the current platform.
fn executable_suffix() -> Result<&'static str> {
    match zed::current_platform().0 {
        Os::Windows => Ok(".exe"),
        _ => Ok(""),
    }
}

/// The binary in a conventional install location, if `$HOME` is known.
///
/// Only `~/.cargo/bin` is tried, because it is the one location the README's own install instruction
/// writes to. Its result is a candidate, not an answer - see the call site, which runs it before
/// believing it.
///
/// `$HOME` comes from the worktree's shell environment, the only handle an extension has on it.
fn cargo_install_location(worktree: &zed::Worktree) -> Option<String> {
    worktree
        .shell_env()
        .into_iter()
        .find(|(name, _)| name == "HOME")
        .map(|(_, home)| format!("{home}/.cargo/bin/{SERVER}"))
}

/// Whether a path is a working `hexaly-lsp`.
///
/// `--version` is the cheap, side-effect-free question that distinguishes "present and executable"
/// from "absent, or a file that cannot run here" - a stale binary for the wrong architecture fails
/// this too, which a filesystem existence check would have passed.
fn responds_to_version(path: &str) -> bool {
    Command::new(path)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status == Some(0))
}
