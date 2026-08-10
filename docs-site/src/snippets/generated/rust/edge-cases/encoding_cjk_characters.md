```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
