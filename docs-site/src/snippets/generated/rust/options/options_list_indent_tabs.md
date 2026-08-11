---
id: fixture_rust_options_list_indent_tabs
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
    let html = r#"<ul><li>Parent<ul><li>Child</li></ul></li></ul>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"list_indent_type":"Tabs"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
