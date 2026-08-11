---
id: fixture_rust_metadata_microdata_schema_article
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
    let html = r#"<html><head><title>Article</title></head><body><article itemscope itemtype="https://schema.org/Article"><h1 itemprop="headline">Breaking News Today</h1><span itemprop="author">Jane Reporter</span><span itemprop="datePublished">2024-04-22</span><div itemprop="articleBody"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
