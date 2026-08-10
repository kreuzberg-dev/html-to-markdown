```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<h1>Closed Heading</h1>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"heading_style":"AtxClosed"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
