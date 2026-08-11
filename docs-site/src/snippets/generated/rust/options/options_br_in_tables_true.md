---
id: fixture_rust_options_br_in_tables_true
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
    let html = r#"<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"br_in_tables":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
