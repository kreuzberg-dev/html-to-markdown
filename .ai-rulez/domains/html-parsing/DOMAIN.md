# HTML Parsing Domain

## Purpose

Foundation of the conversion pipeline: HTML parser selection, DOM tree construction, and tree traversal infrastructure.

## Key Areas

- **Parser backends**: html5ever (HTML5 spec compliance, malformed HTML recovery) and tl/astral-tl (lightweight, fast)
- **DOM traversal**: depth-first tree walking via visitor pattern, parent/child/sibling navigation
- **Node types**: element nodes (60+ tags), text nodes, comment nodes, document/fragment nodes
- **Text extraction**: text content from subtrees, configurable whitespace handling (preserve, minimal, collapse)
- **Attribute access**: by name, iteration, class checking, case-insensitive per HTML spec
- **Safety constraints**: depth limits, size limits, binary data rejection, encoding detection

## Architecture

Parser infrastructure in `converter/tier1/` (byte-scanner) and `converter/main.rs` (`walk_node`, the
recursive DOM-walk / Tier-2 path). Optional caller-supplied traversal hooks via the `HtmlVisitor` trait in
`visitor/traits.rs` (no `DomWalker` trait exists). Element classification into Block, Inline, Void,
FormControl, Semantic categories. Configuration through `ConversionOptions` (parser type, encoding,
whitespace mode, `max_depth`); there is no configurable input size limit.

## Dependencies

- Upstream: html5ever, astral-tl, encoding_rs
- Downstream: Conversion Algorithms domain, Safety-Sanitization domain
