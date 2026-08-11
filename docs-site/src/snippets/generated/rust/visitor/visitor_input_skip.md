---
id: fixture_rust_visitor_input_skip
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
    let html = r#"<p>Sign up:</p><input type="text" name="email" placeholder="your@email.com"><input type="checkbox" name="agree"><p>Continue</p>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_input(&mut self, _ctx: &NodeContext, _input_type: &str, _name: Option<&str>, _value: Option<&str>) -> VisitResult {
            VisitResult::Skip
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
