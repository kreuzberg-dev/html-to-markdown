---
id: fixture_rust_options_autolinks_false
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
    let html = r#"<p><a href='https://example.com'>https://example.com</a></p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"autolinks":false}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
