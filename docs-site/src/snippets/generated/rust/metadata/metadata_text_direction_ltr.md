```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
