---
id: fixture_rust_blockquote_multiple_paragraphs
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
