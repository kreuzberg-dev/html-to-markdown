```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
