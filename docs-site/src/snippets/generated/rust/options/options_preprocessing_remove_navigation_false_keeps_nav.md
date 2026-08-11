---
id: fixture_rust_options_preprocessing_remove_navigation_false_keeps_nav
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
    let html = r#"<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>"#;
    let options_json: serde_json::Value = serde_json::from_str(r#"{"preprocessing":{"remove_navigation":false}}"#).unwrap();
    let options: ConversionOptions = serde_json::from_value(options_json).unwrap();
    let _ = convert(html, Some(options.clone()));
}

```
