```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<article><h1>Article</h1><p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table><ul><li>Item 1</li><li>Item 2</li></ul></article>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
