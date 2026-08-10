```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
