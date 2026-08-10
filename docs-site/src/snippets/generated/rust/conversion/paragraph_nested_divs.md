```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<div><div><p>Nested text</p></div></div>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
