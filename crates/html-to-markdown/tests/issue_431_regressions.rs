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
fn span_paragraph_after_table_in_blockquote_not_glued_to_delimiter_issue_431() {
    let html = r"<blockquote>
  <table>
    <tr>
      <td>A</td>
      <td>B</td>
    </tr>
  </table>
  <p><span>After</span></p>
</blockquote>";

    let result = convert(html, Some(normalized())).unwrap();

    // ~keep The paragraph must not be appended to the delimiter row.
    assert!(
        !result.contains("|After"),
        "`After` must not be glued to the table delimiter row: {result:?}"
    );
    // ~keep `After` lands on its own quoted line (a blank quoted line is optional).
    assert!(
        result.contains("> After"),
        "Expected `After` on its own quoted line: {result:?}"
    );
}

#[test]
fn plain_paragraph_after_table_in_blockquote_still_works_issue_431() {
    // ~keep The <p>After</p> (no span) case already worked; guard against regression.
    let html = r"<blockquote>
  <table><tr><td>A</td><td>B</td></tr></table>
  <p>After</p>
</blockquote>";

    let result = convert(html, Some(normalized())).unwrap();
    assert!(!result.contains("|After"), "{result:?}");
    assert!(result.contains("> After"), "{result:?}");
}

#[test]
fn span_paragraph_after_table_outside_blockquote_still_works_issue_431() {
    // ~keep Outside a blockquote the same structure already worked; guard it.
    let html = r"<table><tr><td>A</td><td>B</td></tr></table><p><span>After</span></p>";

    let result = convert(html, Some(normalized())).unwrap();
    assert!(!result.contains("|After"), "{result:?}");
    assert!(result.contains("After"), "{result:?}");
}
