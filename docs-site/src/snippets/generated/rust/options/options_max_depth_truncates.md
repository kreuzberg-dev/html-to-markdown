---
id: fixture_rust_options_max_depth_truncates
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
    let html = r#"<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"max_depth":3}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
