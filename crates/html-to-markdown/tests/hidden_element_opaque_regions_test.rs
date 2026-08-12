// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

//! Regression coverage for CSS-hidden content leaking into the output when the
//! hidden element's subtree contains a `</tag>` (or `<tag>`) sequence that is
//! not markup: inside an HTML comment, inside a quoted attribute value, or
//! inside a raw-text body.

use html_to_markdown_rs::ConversionOptions;

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, Some(ConversionOptions::default()))
        .expect("conversion failed")
        .content
        .unwrap_or_default()
        .trim()
        .to_string()
}

#[test]
fn should_not_end_the_hidden_element_at_a_close_tag_inside_a_comment() {
    let html = concat!(
        "<p>A</p>",
        r#"<div style="display:none">SECRET<!-- </div> -->MORE-SECRET</div>"#,
        "<p>B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "A\n\nB", "a commented-out close tag ended the hidden scan");
    assert!(!result.contains("SECRET"), "hidden text leaked: {result:?}");
    assert!(!result.contains("-->"), "comment delimiter leaked: {result:?}");
}

#[test]
fn should_not_end_the_hidden_element_at_a_close_tag_inside_a_quoted_attribute() {
    let html = concat!(
        "<p>A</p>",
        r#"<div style="display:none">SECRET<span title="</div>">x</span>MORE-SECRET</div>"#,
        "<p>B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "A\n\nB", "a quoted attribute value ended the hidden scan");
    assert!(!result.contains("SECRET"), "hidden text leaked: {result:?}");
    assert!(!result.contains('"'), "attribute fragment leaked: {result:?}");
}

#[test]
fn should_not_end_the_hidden_element_at_a_close_tag_inside_a_json_ld_script() {
    let html = concat!(
        "<p>A</p>",
        r#"<div style="display:none">"#,
        r#"<script type="application/ld+json">{"a":"</div>"}</script>"#,
        "SECRET</div>",
        "<p>B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "A\n\nB", "a raw-text script body ended the hidden scan");
    assert!(!result.contains("SECRET"), "hidden text leaked: {result:?}");
    assert!(!result.contains('}'), "script body leaked: {result:?}");
}

#[test]
fn should_keep_the_following_sibling_when_a_comment_holds_an_unbalanced_open_tag() {
    let html = concat!(
        r#"<div id="wrap"><div style="display:none">S<!-- <div> --></div>"#,
        "<p>VISIBLE</p></div><p>after</p>"
    );
    let result = convert(html);

    // ~keep Exact equality is the whole assertion here: the failure mode is DROPPING
    // ~keep the visible sibling, which an "absence of secret" check would pass.
    assert_eq!(
        result, "VISIBLE\n\nafter",
        "a commented-out open tag inflated the depth"
    );
}

#[test]
fn should_keep_the_hidden_scan_alive_across_a_literal_less_than_in_text() {
    let html = concat!(
        "<p>A</p>",
        r#"<div style="display:none">SECRET a < b</div>"#,
        "<p>B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "A\n\nB", "a literal `<` in text derailed the hidden scan");
    assert!(!result.contains("SECRET"), "hidden text leaked: {result:?}");
}

#[test]
fn should_still_strip_a_hidden_element_that_nests_the_same_tag_name() {
    let html = concat!(
        "<p>A</p>",
        r#"<div style="display:none">S1<div>S2</div>LEAKED-VISIBLE-TEXT</div>"#,
        "<p>B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "A\n\nB", "plain same-name nesting regressed");
    for leaked in ["S1", "S2", "LEAKED-VISIBLE-TEXT"] {
        assert!(!result.contains(leaked), "{leaked} leaked: {result:?}");
    }
}

#[test]
fn should_still_terminate_a_script_at_its_first_close_tag() {
    // ~keep Pins the per-function discipline: raw-text bodies are character data,
    // ~keep so the script terminator stays first-match. Depth counting here would
    // ~keep run past `</script>` and swallow the rest of the document.
    let html = concat!(
        r#"<script>var s = "<script>"; var leaked = 1;</script>"#,
        "<p>Real content here</p>"
    );
    let result = convert(html);

    assert_eq!(result, "Real content here", "raw-text script terminator regressed");
    assert!(!result.contains("var s"), "script body leaked: {result:?}");
    assert!(!result.contains("var leaked"), "script body leaked: {result:?}");
}
