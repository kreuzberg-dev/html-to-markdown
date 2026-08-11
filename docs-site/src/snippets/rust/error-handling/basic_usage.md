```rust
use html_to_markdown_rs::convert;
use html_to_markdown_rs::error::ConversionError;

fn main() {
    // Binary data (detected via magic bytes) is rejected before parsing.
    let html = "%PDF-1.4 not actually HTML";

    match convert(html, None) {
        Ok(result) => println!("{}", result.content.unwrap_or_default()),
        Err(ConversionError::InvalidInput(message)) => {
            eprintln!("invalid input: {message}");
        }
        Err(ConversionError::ParseError(message)) => {
            eprintln!("parse error: {message}");
        }
        Err(other) => eprintln!("conversion failed: {other}"),
    }
}
```
