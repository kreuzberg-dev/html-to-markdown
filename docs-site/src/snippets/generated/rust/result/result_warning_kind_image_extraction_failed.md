---
id: fixture_rust_result_warning_kind_image_extraction_failed
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Text<img src="data:BADMIME" alt="broken">end</p>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_images":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
