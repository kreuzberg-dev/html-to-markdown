---
id: fixture_rust_malformed_bogus_comment_triple_dash
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
    let html = r#"<h1>One</h1>
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
