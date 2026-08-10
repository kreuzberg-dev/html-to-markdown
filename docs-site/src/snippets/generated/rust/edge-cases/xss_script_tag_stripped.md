```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
