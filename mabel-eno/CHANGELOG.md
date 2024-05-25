# Changelog

## 0.4.2

- Implement missing optional_embed/optional_flag methods on Document (16e5fbf)

## 0.4.1

- Fix missing line-break regression in document snippet printing (56b75a4)

## 0.4.0

- Drop support for associated comments (454cc7c)
- Drop support for continuations (ce5f110)

## 0.3.0

- Introduce snippets with gutter, default printer, various refactoring (32e6d15, 679c64b)
- Use dynamic len_utf8 char index ranges where still incorrectly hardcoded (fdffbe1)

## 0.2.1

Fixes a tiny critical issue discovered right after the `0.2.0` release. (e81de3f)

## 0.2.0

First versioned release on the occasion of some major additions:

- AST only stores range indices for all tokens now (instead of the tokens
  themselves) and creates string slices when key, values, etc. are fetched.
- AST now maps byte indices for absolutely all tokens in the document,
  allowing verbatim reconstruction of the document from the AST itself.
- The newly introduced `Document.print()`, along with its configurable
  printers (`HtmlPrinter`, `TerminalPrinter`, `TextPrinter`) allows obtaining
  a syntax-highlighted version of the input document for different media.