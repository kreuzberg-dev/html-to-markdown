```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<a href="https://old-cdn.com/file.pdf">Download</a>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_link(&mut self, _ctx: &NodeContext, _href: &str, text: &str, _title: Option<&str>) -> VisitResult {
            VisitResult::Custom(format!("[{text}](https://new-cdn.com/file.pdf)"))
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
