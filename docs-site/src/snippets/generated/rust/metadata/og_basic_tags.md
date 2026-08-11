---
id: fixture_rust_og_basic_tags
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
    let html = r#"<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"><meta property="og:description" content="OG description text."><meta property="og:image" content="https://example.com/image.jpg"></head><body><p>Content</p></body></html>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
