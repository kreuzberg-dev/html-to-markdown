---
id: fixture_rust_link_anchor_fragment
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
    let html = r##"<a href="#section">Jump to section</a>"##;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
