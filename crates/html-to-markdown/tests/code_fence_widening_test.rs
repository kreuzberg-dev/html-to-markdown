// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for CORE-CRIT #11: a fenced code block's fence must be strictly longer
//! than the longest run of the fence character (backtick or tilde) found in the block's
//! content, per `CommonMark` 4.5. Previously the fence was hardcoded to exactly three
//! characters, so content containing a triple-backtick run terminated the fence early and
//! corrupted the rest of the document.

use html_to_markdown_rs::{CodeBlockStyle, ConversionOptions};

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, None)
        .expect("conversion should not fail")
        .content
        .unwrap_or_default()
}

fn convert_tildes(html: &str) -> String {
    let options = ConversionOptions {
        code_block_style: CodeBlockStyle::Tildes,
        ..ConversionOptions::default()
    };
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should not fail")
        .content
        .unwrap_or_default()
}

#[test]
fn should_use_four_backtick_fence_when_content_has_three_consecutive_backticks() {
    let html = "<pre><code>a\n```\nb</code></pre>";
    let result = convert(html);
    assert_eq!(result, "````\na\n```\nb\n````\n");
}

#[test]
fn should_use_five_backtick_fence_when_content_has_four_consecutive_backticks() {
    let html = "<pre><code>a\n````\nb</code></pre>";
    let result = convert(html);
    assert_eq!(result, "`````\na\n````\nb\n`````\n");
}

#[test]
fn should_use_six_backtick_fence_when_content_has_five_consecutive_backticks() {
    let html = "<pre><code>a\n`````\nb</code></pre>";
    let result = convert(html);
    assert_eq!(result, "``````\na\n`````\nb\n``````\n");
}

#[test]
fn should_use_three_backtick_fence_when_content_has_no_backticks() {
    let html = "<pre><code>plain content</code></pre>";
    let result = convert(html);
    assert_eq!(result, "```\nplain content\n```\n");
}

#[test]
fn should_use_four_tilde_fence_when_content_has_three_consecutive_tildes() {
    let html = "<pre><code>a\n~~~\nb</code></pre>";
    let result = convert_tildes(html);
    assert_eq!(result, "~~~~\na\n~~~\nb\n~~~~\n");
}

#[test]
fn should_ignore_backtick_runs_when_measuring_tilde_fence() {
    let html = "<pre><code>``````</code></pre>";
    let result = convert_tildes(html);
    assert_eq!(result, "~~~\n``````\n~~~\n");
}

#[test]
fn should_keep_single_backtick_delimiter_when_content_has_only_a_double_backtick_run() {
    // ~keep CommonMark closes a code span at a run of the SAME length as the opener (6.1), so
    // ~keep a run of 2 never collides with a length-1 delimiter; widening to 3 would over-escape.
    let html = "<p><code>x``y</code></p>";
    let result = convert(html);
    assert_eq!(result, "`x``y`\n");
}

#[test]
fn should_widen_inline_code_delimiter_when_content_has_both_single_and_double_backtick_runs() {
    // ~keep the genuine latent bug: content mixing a lone backtick (run length 1) with a
    // ~keep double-backtick run (length 2) needs a length-3 delimiter, since lengths 1 and 2
    // ~keep both collide with runs already present in the content.
    let html = "<p><code>x`y``z</code></p>";
    let result = convert(html);
    assert_eq!(result, "```x`y``z```\n");
}
