---
id: fixture_rust_result_tables_without_structure_flag
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
    let html = r#"<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
