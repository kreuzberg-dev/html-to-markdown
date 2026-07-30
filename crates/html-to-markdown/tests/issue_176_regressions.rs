// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

//! Regression tests for issue #176: Newlines not preserved with adjacent blockquotes

#[test]
fn test_strong_blockquote_strong_newlines() {
    fn convert(
        html: &str,
        opts: Option<html_to_markdown_rs::ConversionOptions>,
    ) -> html_to_markdown_rs::error::Result<String> {
        html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
    }

    let html = r"<strong>2. Point two</strong><blockquote>Option Explicit
Sub Test()
    ' code here
End Function</blockquote><strong>3. Point three</strong>";

    let markdown = convert(html, None).unwrap();

    println!("Actual output:\n{markdown}");
    println!("---");

    assert!(
        markdown.contains("**2. Point two**\n\n>"),
        "Should have blank line between strong and blockquote. Got: {markdown:?}"
    );
    assert!(
        markdown.contains("End Function\n\n**3. Point three**"),
        "Should have blank line between blockquote and next strong. Got: {markdown:?}"
    );
}

#[test]
fn test_paragraph_blockquote_paragraph_newlines() {
    fn convert(
        html: &str,
        opts: Option<html_to_markdown_rs::ConversionOptions>,
    ) -> html_to_markdown_rs::error::Result<String> {
        html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
    }

    let html = r"<p>First paragraph</p><blockquote>A quote</blockquote><p>Second paragraph</p>";

    let markdown = convert(html, None).unwrap();

    println!("Actual output:\n{markdown}");

    // ~keep Should have single newline before blockquote (CommonMark spec)
    // ~keep and blank line after blockquote
    assert!(
        markdown.contains("First paragraph\n>"),
        "Should have single newline between p and blockquote (CommonMark compliance)"
    );
    assert!(
        markdown.contains("A quote\n\n"),
        "Should have blank line after blockquote"
    );
}
