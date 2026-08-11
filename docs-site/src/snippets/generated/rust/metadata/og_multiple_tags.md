---
id: fixture_rust_og_multiple_tags
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
    let html = r#"<html><head><meta property="og:title" content="Article Title"><meta property="og:type" content="article"><meta property="og:url" content="https://example.com/article"><meta property="og:site_name" content="Example Site"><meta property="og:description" content="An interesting article."><meta property="og:image" content="https://example.com/article.jpg"></head><body><article><p>Article content here.</p></article></body></html>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
