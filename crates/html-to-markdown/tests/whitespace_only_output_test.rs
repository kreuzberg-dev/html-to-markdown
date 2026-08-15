// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::{ConversionOptions, OutputFormat, options::TierStrategy};

const WHITESPACE_ONLY_INPUTS: [&str; 6] = [
    "<p></p>",
    "<p> </p>",
    "<p>   </p>",
    "<div>   </div>",
    "<span>   </span>",
    "   ",
];

// ~keep These assertions are byte-exact on purpose: the bug they pin produced "  \n"
// ~keep (a bare markdown hard break with no content) instead of "", and every generated
// ~keep e2e assertion trimmed the output, so nothing in the suite ever caught it.

#[test]
fn should_return_empty_for_empty_paragraph() {
    assert_eq!(convert("<p></p>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_paragraph_with_one_space() {
    assert_eq!(convert("<p> </p>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_paragraph_with_three_spaces() {
    assert_eq!(convert("<p>   </p>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_div_with_three_spaces() {
    assert_eq!(convert("<div>   </div>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_span_with_three_spaces() {
    assert_eq!(convert("<span>   </span>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_bare_whitespace_text_input() {
    assert_eq!(convert("   ", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_bare_whitespace_text_input_of_every_width() {
    for width in 1..=8 {
        let html = " ".repeat(width);
        assert_eq!(
            convert(&html, None).unwrap(),
            "",
            "width {width} must collapse to empty"
        );
    }
}

#[test]
fn should_return_empty_for_paragraph_whitespace_of_every_width() {
    for width in 0..=8 {
        let html = format!("<p>{}</p>", " ".repeat(width));
        assert_eq!(
            convert(&html, None).unwrap(),
            "",
            "width {width} must collapse to empty"
        );
    }
}

#[test]
fn should_return_empty_for_mixed_whitespace_input() {
    assert_eq!(convert("<p> \t \t </p>", None).unwrap(), "");
    assert_eq!(convert(" \t \n ", None).unwrap(), "");
    assert_eq!(convert("<div> <span>  </span> </div>", None).unwrap(), "");
}

#[test]
fn should_return_empty_for_whitespace_only_input_in_plain_format() {
    let plain = ConversionOptions {
        output_format: OutputFormat::Plain,
        ..Default::default()
    };
    assert_eq!(convert("<p>   </p>", Some(plain.clone())).unwrap(), "");
    assert_eq!(convert("<span>   </span>", Some(plain.clone())).unwrap(), "");
    assert_eq!(convert("   ", Some(plain)).unwrap(), "");
}

#[test]
fn should_return_empty_for_whitespace_only_input_on_the_tier2_path() {
    for html in WHITESPACE_ONLY_INPUTS {
        let options = ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..Default::default()
        };
        assert_eq!(convert(html, Some(options)).unwrap(), "", "tier-2 output for {html:?}");
    }
}

#[test]
fn should_return_empty_for_whitespace_only_input_on_the_default_path() {
    for html in WHITESPACE_ONLY_INPUTS {
        assert_eq!(convert(html, None).unwrap(), "", "default output for {html:?}");
    }
}

// ~keep The emptiness guard must stay whitespace-complete WITHOUT eating the two-space
// ~keep hard break markdown uses on content-bearing lines.

#[test]
fn should_preserve_hard_break_on_content_bearing_lines() {
    assert_eq!(
        convert("<div><span>First</span><br><span>Second</span></div>", None).unwrap(),
        "First  \nSecond\n"
    );
}

#[test]
fn should_preserve_trailing_hard_break_after_content() {
    assert_eq!(convert("<p>Only<br></p>", None).unwrap(), "Only  \n");
}

#[test]
fn should_keep_hard_break_after_content_but_not_on_a_blank_line() {
    // ~keep The break after "text" survives; the blank first line keeps no invisible
    // ~keep trailing spaces, because a hard break needs content on its own line.
    assert_eq!(convert("<p> <br>text<br> </p>", None).unwrap(), "\ntext  \n");
}

#[test]
fn should_not_append_a_blank_hard_break_line_after_trailing_whitespace() {
    assert_eq!(convert("<p>x</p>   ", None).unwrap(), "x\n");
}
