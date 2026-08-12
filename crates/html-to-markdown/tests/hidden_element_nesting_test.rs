// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

//! Regression coverage for CSS-hidden content leaking into the output when a
//! hidden element contains a nested element of the *same* tag name.

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
fn should_strip_whole_hidden_element_when_it_nests_the_same_tag_name() {
    let html = r#"<p>A<div style="display:none">SECRET1<div>SECRET2</div>LEAKED-VISIBLE-TEXT</div>B</p>"#;
    let result = convert(html);

    assert_eq!(result, "AB", "hidden div must be removed in full");
    assert!(
        !result.contains("LEAKED-VISIBLE-TEXT"),
        "tail of a hidden element leaked: {result:?}"
    );
    assert!(!result.contains("SECRET1"), "hidden text leaked: {result:?}");
    assert!(!result.contains("SECRET2"), "hidden text leaked: {result:?}");
}

#[test]
fn should_strip_wikipedia_start_date_template_without_leaving_stray_paren() {
    let html = concat!(
        "<p>Released",
        r#"<span style="display: none">&#160;(<span class="bday">2012-01-19</span>)</span>"#,
        "</p>"
    );
    let result = convert(html);

    assert_eq!(result, "Released", "hidden span must be removed in full");
    assert!(!result.contains(')'), "stray closing paren survived: {result:?}");
    assert!(!result.contains("2012-01-19"), "hidden date leaked: {result:?}");
}

#[test]
fn should_strip_hidden_element_nested_three_levels_deep_in_the_same_tag() {
    let html = concat!(
        "<p>A",
        r#"<div style="display:none">L0<div>L1<div>L2<div>L3</div>T3</div>T2</div>T1</div>"#,
        "B</p>"
    );
    let result = convert(html);

    assert_eq!(result, "AB", "every level of the hidden subtree must be removed");
    for leaked in ["L0", "L1", "L2", "L3", "T1", "T2", "T3"] {
        assert!(!result.contains(leaked), "{leaked} leaked: {result:?}");
    }
}

#[test]
fn should_strip_hidden_element_regardless_of_tag_name_case() {
    let html = r#"<p>A<DIV style="display:none">x<div>y</div>z</DIV>B</p>"#;
    let result = convert(html);

    assert_eq!(result, "AB", "case-insensitive tag matching must close the hidden div");
    for leaked in ["x", "y", "z"] {
        assert!(!result.contains(leaked), "{leaked} leaked: {result:?}");
    }
}

#[test]
fn should_not_increment_depth_for_a_self_closing_same_name_tag() {
    let html = r#"<p>A<div style="display:none">S1<div/>S2</div>B</p>"#;
    let result = convert(html);

    assert_eq!(result, "AB", "a self-closing nested tag must not extend the scan");
    assert!(!result.contains("S1") && !result.contains("S2"), "leak: {result:?}");
}

#[test]
fn should_keep_visible_content_when_hidden_element_is_never_closed() {
    let html = r#"<p>A</p><div style="display:none">DANGLING"#;
    let result = convert(html);

    assert!(result.starts_with('A'), "visible content lost: {result:?}");
    assert!(!result.contains("<div"), "open tag survived: {result:?}");
}

#[test]
fn should_still_strip_script_whose_body_contains_a_literal_script_open_tag() {
    let html = concat!(
        r#"<script>var s = "<script>"; var leaked = 1;</script>"#,
        "<p>Real content here</p>"
    );
    let result = convert(html);

    assert_eq!(result, "Real content here", "raw-text script terminator regressed");
    assert!(!result.contains("var s"), "script body leaked: {result:?}");
    assert!(!result.contains("var leaked"), "script body leaked: {result:?}");
}

#[test]
fn should_still_strip_style_whose_body_contains_a_literal_style_open_tag() {
    let html = concat!(
        r#"<style>.a::after { content: "<style>"; } .b { color: red; }</style>"#,
        "<p>Real content here</p>"
    );
    let result = convert(html);

    assert_eq!(result, "Real content here", "raw-text style terminator regressed");
    assert!(!result.contains("color: red"), "style body leaked: {result:?}");
}

#[test]
fn should_still_strip_plain_nested_script_and_style_blocks() {
    let html = concat!(
        "<style>body { margin: 0; }</style>",
        "<div><script>var x = 1;</script><p>Visible</p></div>"
    );
    let result = convert(html);

    assert_eq!(result, "Visible");
    assert!(
        !result.contains("margin") && !result.contains("var x"),
        "leak: {result:?}"
    );
}
