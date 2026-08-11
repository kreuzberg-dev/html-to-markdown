---
id: fixture_rust_structure_code_block
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
    let html = r#"<p>Example code:</p><pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"include_document_structure":true}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
