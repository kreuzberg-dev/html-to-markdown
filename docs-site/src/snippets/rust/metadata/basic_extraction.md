```rust
use html_to_markdown_rs::{convert, ConversionOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"<html><head><title>My Page</title></head>
    <body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>"#;

    let options = ConversionOptions::builder()
        .extract_metadata(true)
        .build();
    let result = convert(html, Some(options))?;
    let markdown = result.content.clone().unwrap_or_default();
    println!("Markdown: {markdown}");
    println!("Title: {:?}", result.metadata.document.title);
    println!("Links: {:?}", result.metadata.links);
    Ok(())
}
```
