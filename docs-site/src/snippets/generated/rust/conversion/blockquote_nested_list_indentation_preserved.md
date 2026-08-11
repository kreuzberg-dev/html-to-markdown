---
id: fixture_rust_blockquote_nested_list_indentation_preserved
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
    let html = r#"<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
