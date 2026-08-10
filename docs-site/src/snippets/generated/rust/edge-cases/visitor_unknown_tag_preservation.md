```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_custom_element(&mut self, _ctx: &NodeContext, _tag_name: &str, _html: &str) -> VisitResult {
            VisitResult::PreserveHtml
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
