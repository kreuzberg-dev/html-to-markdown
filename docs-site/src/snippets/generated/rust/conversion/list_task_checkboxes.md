---
id: fixture_rust_list_task_checkboxes
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
    let html = r#"<ul><li><input type="checkbox" checked> Done task</li><li><input type="checkbox"> Pending task</li></ul>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
