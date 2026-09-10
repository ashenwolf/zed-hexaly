# Changelog

## 0.1.0 — unreleased

Initial release: Hexaly Modeler language support (`.hxm`).

- New tree-sitter grammar for the language, validated against a production model, all 66
  example models shipped with Hexaly 14.0, and the official HXM BNF, released separately as
  [tree-sitter-hexaly](https://github.com/ashenwolf/tree-sitter-hexaly)
- Highlighting, bracket matching, indentation, outline and text-object queries for Zed
- Launches [hexaly-lsp](https://github.com/ashenwolf/hexaly-lsp) for diagnostics, completion,
  hover and signature help. The server is a separate executable, not bundled: it speaks plain LSP
  over stdio and is not Zed-specific.
