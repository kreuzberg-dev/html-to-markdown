---
id: fixture_rust_visitor_custom_element_with_nesting
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<div><custom-widget data-value="123"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_custom_element(&mut self, _ctx: &NodeContext, _tag_name: &str, _html: &str) -> VisitResult {
            VisitResult::Custom("[CUSTOM WIDGET]".to_string())
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
