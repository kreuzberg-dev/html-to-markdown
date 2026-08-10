```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"include_document_structure":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
