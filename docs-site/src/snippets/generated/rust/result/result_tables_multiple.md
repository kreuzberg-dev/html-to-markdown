```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"include_document_structure":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
