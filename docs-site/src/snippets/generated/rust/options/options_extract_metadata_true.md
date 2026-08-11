---
id: fixture_rust_options_extract_metadata_true
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
    let html = r#"<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
