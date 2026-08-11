---
id: fixture_rust_structured_data_multiple_json_ld
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
    let html = r#"<html><head><title>Shop Page</title><script type="application/ld+json">{"@context":"https://schema.org","@type":"Product","name":"Widget","price":"9.99"}</script><script type="application/ld+json">{"@context":"https://schema.org","@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Home"}]}</script></head><body><h1>Widget</h1><p>A great widget for all purposes.</p></body></html>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
