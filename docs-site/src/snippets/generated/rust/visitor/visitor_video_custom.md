---
id: fixture_rust_visitor_video_custom
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
    let html = r#"<p>Watch our tutorial:</p><video src="tutorial.mp4" width="320" height="240" controls></video><p>Great content!</p>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_video(&mut self, _ctx: &NodeContext, src: Option<&str>) -> VisitResult {
            let src = src.unwrap_or_default();
            VisitResult::Custom(format!("[VIDEO: {src}]"))
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
