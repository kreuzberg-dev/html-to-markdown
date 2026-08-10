```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult};

fn main() {
    let html = r#"<form><label>Username: <input type="text" name="username" value=""></label><label>Password: <input type="password" name="password"></label></form>"#;
    let mut options: ConversionOptions = Default::default();
    #[derive(Debug)]
    struct _TestVisitor;
    impl HtmlVisitor for _TestVisitor {
        fn visit_input(&mut self, _ctx: &NodeContext, input_type: &str, _name: Option<&str>, _value: Option<&str>) -> VisitResult {
            VisitResult::Custom(format!("[INPUT:{input_type}]"))
        }
    }
    let visitor = std::sync::Arc::new(std::sync::Mutex::new(_TestVisitor));
    options.visitor = Some(visitor);
    let _ = convert(html, Some(options));
}

```
