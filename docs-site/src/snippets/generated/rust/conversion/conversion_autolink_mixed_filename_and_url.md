---
id: fixture_rust_conversion_autolink_mixed_filename_and_url
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r#"<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>"#;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
