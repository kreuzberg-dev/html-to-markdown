```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_definition_description(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            VisitResult::Custom(format!("> {text}"))
        }
        fn visit_definition_term(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
            VisitResult::Custom(format!("### {text}"))
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
