# zed-hexaly

Hexaly language support for [Zed](https://zed.dev) — syntax highlighting for Hexaly Modeler
models (`.hxm`).

Ships a purpose-built tree-sitter grammar for the language,
because none existed publicly: [tree-sitter-hexaly](https://github.com/ashenwolf/tree-sitter-hexaly).
Zed cannot use the TextMate grammar from the official
[Hexaly VS Code extension](https://marketplace.visualstudio.com/items?itemName=hexaly.hexaly),
so highlighting had to be rebuilt on tree-sitter.

## Status

Syntax highlighting only. A language server (diagnostics, go-to-definition, completions) is
the next milestone and is not implemented yet.

What works today:

- Highlighting of modeling constructs — `constraint`, `minimize`/`maximize`, the `<-`
  decision operator, indexed declarations over an index space, and `sum`/`min`/`max`
  aggregates
- Bracket matching, auto-indent, and text objects
- An outline listing model decisions alongside functions, since in a Hexaly model the
  decisions are the structure

## Install

Not yet in the Zed extension registry. To use it now, install as a dev extension:

1. Clone this repository
2. In Zed, run `zed: install dev extension` from the command palette
3. Select the cloned directory

## Development

This repository holds the Zed queries in `languages/hexaly`. The grammar is a separate
repository, [tree-sitter-hexaly](https://github.com/ashenwolf/tree-sitter-hexaly), pinned by
`rev` in `extension.toml`: Zed clones it and compiles its generated `src/parser.c` directly,
so keeping it apart is what keeps generated C out of this repo. A grammar change means
landing it there and bumping `rev` here.

The grammar was validated by parsing a production Hexaly model (12 files, ~2500 lines) and
all 66 example models shipped with Hexaly 14.0 with zero errors, and corrected against the
official
[HXM BNF](https://www.hexaly.com/docs/last/modelerreference/appendix.html#bnf-syntax); the
findings from that pass are kept in the grammar repository's `docs/`.

## License

MIT — see [LICENSE](LICENSE). Not affiliated with or endorsed by Hexaly.
