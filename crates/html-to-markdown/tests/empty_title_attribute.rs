// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! An empty `title=""` must be treated as absent.
//!
//! `[text](url "")` and `![alt](src "")` carry no information, and no Markdown serializer
//! round-trips them: re-rendering the output drops the empty title, so converting that
//! result again produces different Markdown than the first pass. Found by the
//! convert/render/convert fixpoint oracle in `roundtrip_fixpoint.rs`.

use html_to_markdown_rs::options::ConversionOptions;

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn should_omit_an_empty_title_on_a_link() {
    assert_eq!(convert(r#"<a href="u" title="">l</a>"#), "[l](u)\n");
}

#[test]
fn should_omit_an_empty_title_on_an_image() {
    assert_eq!(convert(r#"<img src="i.png" alt="a" title="">"#), "![a](i.png)\n");
}

#[test]
fn should_keep_a_non_empty_title() {
    // ~keep The counterweight: only the empty case changes.
    assert_eq!(convert(r#"<a href="u" title="t">l</a>"#), "[l](u \"t\")\n");
    assert_eq!(convert(r#"<img src="i.png" alt="a" title="t">"#), "![a](i.png \"t\")\n");
    // ~keep Whitespace is deliberately still a title: only a genuinely empty attribute is
    // ~keep treated as absent, so this stays a narrow change rather than a trimming policy.
    assert_eq!(convert(r#"<a href="u" title=" ">l</a>"#), "[l](u \" \")\n");
}
