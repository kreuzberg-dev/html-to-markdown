```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Before<img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel">After</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_images":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
