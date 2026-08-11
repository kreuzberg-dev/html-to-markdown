---
id: fixture_rust_metadata_dublin_core
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
    let html = r#"<html><head><title>Scholarly Work</title><meta name="DC.title" content="Principles of Knowledge Management"><meta name="DC.creator" content="Dr. Alice Johnson"><meta name="DC.date" content="2023-06-15"><meta name="DC.subject" content="Knowledge Management"><meta name="DC.publisher" content="Academic Press"></head><body><p>This is a scholarly article.</p></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
