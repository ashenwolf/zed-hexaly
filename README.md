# zed-hexaly

Hexaly language support for [Zed](https://zed.dev) — syntax highlighting for Hexaly Modeler
models (`.hxm`).

Ships a purpose-built tree-sitter grammar for the language,
because none existed publicly: [tree-sitter-hexaly](https://github.com/ashenwolf/tree-sitter-hexaly).
Zed cannot use the TextMate grammar from the official
[Hexaly VS Code extension](https://marketplace.visualstudio.com/items?itemName=hexaly.hexaly),
so highlighting had to be rebuilt on tree-sitter.

## Status

Syntax highlighting, plus a language server ([hexaly-lsp](https://github.com/ashenwolf/hexaly-lsp))
providing diagnostics, completion, hover and signature help.

What works today:

- Highlighting of modeling constructs — `constraint`, `minimize`/`maximize`, the `<-`
  decision operator, indexed declarations over an index space, and `sum`/`min`/`max`
  aggregates
- Bracket matching, auto-indent, and text objects
- An outline listing model decisions alongside functions, since in a Hexaly model the
  decisions are the structure
- Diagnostics: every syntax error at once from the grammar, and structural errors (duplicate
  declarations, unresolvable `use`) from a local Hexaly installation when there is one
- Completion, hover and signature help over 396 standard-library symbols and the declarations in
  the file being edited

## Install

Not yet in the Zed extension registry. To use it now, install as a dev extension:

1. Install the language server: `cargo install --git https://github.com/ashenwolf/hexaly-lsp`
2. Clone this repository
3. In Zed, run `zed: install dev extension` from the command palette
4. Select the cloned directory

The server is found on `$PATH`, then at `~/.cargo/bin/hexaly-lsp`. If neither works, set the path
explicitly:

```json
{
  "lsp": {
    "hexaly-lsp": { "binary": { "path": "/path/to/hexaly-lsp" } }
  }
}
```

On a **remote (SSH) project** the server runs on the remote host, so install it there — and put any
explicit path in that project's `.zed/settings.json` rather than your global settings, which are
shared with local projects and would point at the wrong filesystem.

Diagnostics from the Hexaly compiler need a Hexaly installation but **no licence**; without one the
server reports so and continues with everything else.

## Development

This repository holds the Zed queries in `languages/hexaly` and the extension itself in `src/`. The
extension is deliberately thin: it answers "where is the language server binary" and nothing else.
Everything about the language lives in
[hexaly-lsp](https://github.com/ashenwolf/hexaly-lsp), which speaks plain LSP over stdio and is not
Zed-specific, so the same server serves Neovim, Helix and Emacs.

Notably absent from the extension: locating the *Hexaly* installation. Zed offers
`worktree.which()`, which would work here and would also strand every other editor, so the server
discovers Hexaly itself.

```sh
cargo build --release --target wasm32-wasip1   # what Zed loads
```

The grammar is a separate repository,
[tree-sitter-hexaly](https://github.com/ashenwolf/tree-sitter-hexaly), pinned by
`rev` in `extension.toml`: Zed clones it and compiles its generated `src/parser.c` directly,
so keeping it apart is what keeps generated C out of this repo. A grammar change means
landing it there and bumping `rev` here.

The grammar was validated by parsing a production Hexaly model (12 files, ~2500 lines) and
all 66 example models shipped with Hexaly 14.0 with zero errors, and corrected against the
official
[HXM BNF](https://www.hexaly.com/docs/last/modelerreference/appendix.html#bnf-syntax) and the
compiler itself; the findings from those passes are kept in the grammar repository's `docs/`.

## License

MIT — see [LICENSE](LICENSE). Not affiliated with or endorsed by Hexaly.
