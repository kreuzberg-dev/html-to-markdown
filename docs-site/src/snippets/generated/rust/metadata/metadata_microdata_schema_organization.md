---
id: fixture_rust_metadata_microdata_schema_organization
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
    let html = r#"<html><head><title>Company</title></head><body><div itemscope itemtype="https://schema.org/Organization"><span itemprop="name">Acme Corp</span><span itemprop="foundingDate">2020</span><span itemprop="url">https://acmecorp.example.com</span><span itemprop="logo">https://acmecorp.example.com/logo.png</span></div></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
