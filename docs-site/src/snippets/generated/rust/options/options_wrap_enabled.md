```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"wrap":true,"wrap_width":40}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
