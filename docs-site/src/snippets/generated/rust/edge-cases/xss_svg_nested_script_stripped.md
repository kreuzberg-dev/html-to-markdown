```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>Before SVG.</p><svg xmlns="http://www.w3.org/2000/svg"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
