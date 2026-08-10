```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"exclude_selectors":[".cookie-banner"]}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
