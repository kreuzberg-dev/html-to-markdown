```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
