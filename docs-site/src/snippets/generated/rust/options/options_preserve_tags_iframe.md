```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"preserve_tags":["iframe"]}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
