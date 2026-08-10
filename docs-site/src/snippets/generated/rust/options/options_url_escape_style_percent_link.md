```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<a href="/file (1).pdf">file</a>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"url_escape_style":"percent"}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
