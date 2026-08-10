```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
