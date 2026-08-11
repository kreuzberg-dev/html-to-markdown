---
id: fixture_rust_twitter_card_tags
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
    let html = r#"<html><head><meta name="twitter:card" content="summary_large_image"><meta name="twitter:site" content="@examplesite"><meta name="twitter:title" content="Twitter Card Title"><meta name="twitter:description" content="Twitter card description."><meta name="twitter:image" content="https://example.com/twitter-image.jpg"></head><body><p>Content</p></body></html>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
