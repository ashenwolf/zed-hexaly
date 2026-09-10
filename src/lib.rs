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

use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

const SERVER: &str = "hexaly-lsp";

struct HexalyExtension;

impl zed::Extension for HexalyExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _server_id: &LanguageServerId,
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

        // Then `$PATH`, which is the documented convention for extensions wrapping a server the
        // user installs themselves.
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
        if let Some(command) = cargo_install_location(worktree) {
            return Ok(zed::Command {
                command,
                args: Vec::new(),
                env: worktree.shell_env(),
            });
        }

        Err(format!(
            "{SERVER} was not found. Install it with `cargo install --git \
             https://github.com/ashenwolf/hexaly-lsp`, then set lsp.{SERVER}.binary.path in your \
             Zed settings if it is still not picked up."
        ))
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

zed::register_extension!(HexalyExtension);

/// The binary in a conventional install location, if `$HOME` is known.
///
/// Only `~/.cargo/bin` is tried. The extension API deliberately exposes no host filesystem access -
/// `read_text_file` is scoped to the worktree - so existence cannot be checked, which means every
/// candidate returned is a guess Zed will fail on if wrong. One well-founded guess is therefore
/// better than a list: `cargo install` is how the README says to install this server, and its
/// location is fixed.
///
/// `$HOME` comes from the worktree's shell environment, the only handle an extension has on it.
fn cargo_install_location(worktree: &zed::Worktree) -> Option<String> {
    worktree
        .shell_env()
        .into_iter()
        .find(|(name, _)| name == "HOME")
        .map(|(_, home)| format!("{home}/.cargo/bin/{SERVER}"))
}
