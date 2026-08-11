---
id: fixture_rust_malformed_missing_block_closing_tags
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
    let html = r#"<div><h1>Title<p>First paragraph<p>Second paragraph</div>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
