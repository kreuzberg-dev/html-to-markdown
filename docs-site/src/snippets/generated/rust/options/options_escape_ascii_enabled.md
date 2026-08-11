---
id: fixture_rust_options_escape_ascii_enabled
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
    let html = r#"<p>Text with # hash and [brackets] and * star</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"escape_ascii":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
