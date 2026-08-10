```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<section><h2>Gallery</h2><figure><img src="photo1.jpg" alt="Photo"><figcaption>Beautiful sunset</figcaption></figure></section>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_figure_end(&mut self, _ctx: &NodeContext, output: &str) -> VisitResult {
            VisitResult::Custom(format!("{output}\n[/FIGURE]\n"))
        }
        fn visit_figure_start(&mut self, _ctx: &NodeContext) -> VisitResult {
            VisitResult::Custom("\n[FIGURE]\n".to_string())
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
