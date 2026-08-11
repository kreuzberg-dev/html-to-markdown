---
id: fixture_rust_options_output_format_plain
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
    let html = r#"<h1>Title</h1><p>Some <strong>bold</strong> text.</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"output_format":"Plain"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
