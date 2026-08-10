```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"strip_tags":["div","span"]}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
