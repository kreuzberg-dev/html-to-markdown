---
id: fixture_rust_options_sup_symbol_caret
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
    let html = r#"<p>x<sup>2</sup></p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"sup_symbol":"^"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
