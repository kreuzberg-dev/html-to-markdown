// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression test for issue #348: custom `HtmlVisitor` returning `VisitResult::Custom`
//! from `visit_element_end` must be honoured when `OutputFormat::Plain`.

#![cfg(feature = "visitor")]

use std::sync::{Arc, Mutex};

use html_to_markdown_rs::visitor::{HtmlVisitor, VisitResult};
use html_to_markdown_rs::{ConversionOptions, NodeContext, OutputFormat};

/// Visitor that accumulates text inside `<custom>` elements and returns the
/// uppercased, bracketed version from `visit_element_end`.
#[derive(Debug, Default)]
struct CustomVisitor {
    accum: Option<String>,
}

impl HtmlVisitor for CustomVisitor {
    fn visit_element_start(&mut self, ctx: &NodeContext) -> VisitResult {
        if ctx.tag_name == "custom" {
            self.accum = Some(String::new());
        }
        VisitResult::Continue
    }

    fn visit_text(&mut self, _ctx: &NodeContext, text: &str) -> VisitResult {
        if let Some(accum) = self.accum.as_mut() {
            accum.push_str(text);
            return VisitResult::Skip;
        }
        VisitResult::Continue
    }

    fn visit_element_end(&mut self, ctx: &NodeContext, _output: &str) -> VisitResult {
        if ctx.tag_name == "custom"
            && let Some(mut accum) = self.accum.take()
        {
            accum.make_ascii_uppercase();
            return VisitResult::Custom(format!(">>{accum}<<"));
        }
        VisitResult::Continue
    }
}

/// `VisitResult::Custom` from `visit_element_end` is honoured for `OutputFormat::Plain`.
#[test]
fn test_visitor_custom_end_result_honoured_for_plain_output() {
    let input = "before <custom>foo</custom> after";
    let options = ConversionOptions {
        output_format: OutputFormat::Plain,
        visitor: Some(Arc::new(Mutex::new(CustomVisitor::default()))),
        ..Default::default()
    };
    let output = html_to_markdown_rs::convert(input, Some(options))
        .expect("conversion must not fail")
        .content
        .expect("content must be present");
    assert_eq!(output, "before >>FOO<< after\n");
}

/// `VisitResult::Skip` from `visit_element_end` suppresses element output for `OutputFormat::Plain`.
#[test]
fn test_visitor_skip_end_result_honoured_for_plain_output() {
    #[derive(Debug, Default)]
    struct DropSpanVisitor;

    impl HtmlVisitor for DropSpanVisitor {
        fn visit_element_end(&mut self, ctx: &NodeContext, _output: &str) -> VisitResult {
            if ctx.tag_name == "span" {
                VisitResult::Skip
            } else {
                VisitResult::Continue
            }
        }
    }

    let input = "<p>keep <span>drop this</span> keep</p>";
    let options = ConversionOptions {
        output_format: OutputFormat::Plain,
        visitor: Some(Arc::new(Mutex::new(DropSpanVisitor))),
        ..Default::default()
    };
    let output = html_to_markdown_rs::convert(input, Some(options))
        .expect("conversion must not fail")
        .content
        .expect("content must be present");
    assert!(!output.contains("drop this"), "got: {output:?}");
    assert!(output.contains("keep"), "got: {output:?}");
}
