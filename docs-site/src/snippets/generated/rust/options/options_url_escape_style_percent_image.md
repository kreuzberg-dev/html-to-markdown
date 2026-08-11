---
id: fixture_rust_options_url_escape_style_percent_image
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
    let html = r#"<img src="/img (1) <draft>.png" alt="alt">"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"url_escape_style":"percent"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
