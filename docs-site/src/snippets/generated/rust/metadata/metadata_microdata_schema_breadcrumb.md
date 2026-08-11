---
id: fixture_rust_metadata_microdata_schema_breadcrumb
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
    let html = r#"<html><head><title>Navigation</title></head><body><nav itemscope itemtype="https://schema.org/BreadcrumbList"><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com"><span itemprop="name">Home</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com/products"><span itemprop="name">Products</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><span itemprop="name">Current Page</span></span></nav></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true,"preprocessing":{"remove_navigation":false}}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
