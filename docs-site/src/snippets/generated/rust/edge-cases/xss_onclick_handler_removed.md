---
id: fixture_rust_xss_onclick_handler_removed
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
    let html = r#"<p><a href="https://example.com" onclick="alert('xss')">Click me</a></p><button onmouseover="steal_data()">Hover me</button>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
