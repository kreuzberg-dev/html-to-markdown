```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"link_style":"Reference"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
