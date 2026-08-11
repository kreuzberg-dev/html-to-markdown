---
id: fixture_rust_metadata_link_type_anchor_classified
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
    let html = r##"<p>Jump to <a href="#section">section</a> below.</p>"##;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"extract_metadata":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
