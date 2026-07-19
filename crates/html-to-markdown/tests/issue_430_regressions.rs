// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::ConversionOptions;
use html_to_markdown_rs::options::WhitespaceMode;

fn normalized() -> ConversionOptions {
    ConversionOptions {
        whitespace_mode: WhitespaceMode::Normalized,
        ..Default::default()
    }
}

#[test]
fn newline_only_span_between_inline_elements_collapses_to_space_issue_430() {
    // ~keep Outlook-style HTML: whitespace split across styled spans. The middle
    // ~keep span holds only a newline, which must collapse to a single space.
    let html = "<p><b><span>1 mezzo</span></b><span>\n</span><span>con carico</span></p>";
    let result = convert(html, Some(normalized())).unwrap();

    assert!(
        result.contains("**1 mezzo** con carico"),
        "Expected a single space between the inline elements: {result:?}"
    );
    assert!(
        !result.contains("**1 mezzo**con carico"),
        "Inline elements must not be glued together: {result:?}"
    );
}

#[test]
fn literal_space_span_still_works_issue_430() {
    // ~keep The literal-space variant already worked; guard against regression.
    let html = "<p><b><span>1 mezzo</span></b><span> </span><span>con carico</span></p>";
    let result = convert(html, Some(normalized())).unwrap();
    assert!(result.contains("**1 mezzo** con carico"), "{result:?}");
}

#[test]
fn newline_only_span_between_bare_inline_spans_collapses_issue_430() {
    let html = "<div><span>a</span><span>\n</span><span>b</span></div>";
    let result = convert(html, Some(normalized())).unwrap();
    assert!(result.contains("a b"), "{result:?}");
    assert!(!result.contains("ab"), "{result:?}");
}
