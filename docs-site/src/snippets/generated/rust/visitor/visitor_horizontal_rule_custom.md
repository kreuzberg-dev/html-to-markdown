---
id: fixture_rust_visitor_horizontal_rule_custom
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
    let html = r#"<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_horizontal_rule(&mut self, _ctx: &NodeContext) -> VisitResult {
            VisitResult::Custom("\n[DIVIDER]\n".to_string())
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
