// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for `CommonMark` spec examples that the `commonmark_spec_fixpoint` oracle
//! (`tests/commonmark_spec_fixpoint.rs`) found unstable in the Lists / List items sections:
//! converting the spec's HTML produced Markdown that, once rendered back to HTML and
//! reconverted, produced a *different* Markdown -- meaning the first conversion was not a
//! fixpoint. Each test here pins the corrected, verified-stable output for one example and
//! also asserts the round-trip property directly, so a future change that reintroduces the
//! instability fails here with a much narrower diff than the full 652-example oracle.

use html_to_markdown_rs::ConversionOptions;

fn options() -> ConversionOptions {
    // ~keep Matches `commonmark_spec_fixpoint.rs::escaping_options` -- the configuration in
    // ~keep which round-trip stability is a real contract for this crate.
    ConversionOptions {
        escape_misc: true,
        escape_asterisks: true,
        escape_underscores: true,
        ..Default::default()
    }
}

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, Some(options()))
        .unwrap()
        .content
        .unwrap_or_default()
}

fn render_to_html(markdown: &str) -> String {
    let mut opts = comrak::Options::default();
    opts.extension.table = true;
    opts.extension.strikethrough = true;
    opts.render.r#unsafe = true;
    comrak::markdown_to_html(markdown, &opts)
}

/// Asserts that converting `html` reaches a fixpoint: reconverting the HTML that a real
/// `CommonMark` renderer produces from the first pass's Markdown yields the identical
/// Markdown again.
fn assert_reaches_fixpoint(html: &str) {
    let first_pass = convert(html);
    let rendered = render_to_html(&first_pass);
    let second_pass = convert(&rendered);
    assert_eq!(
        first_pass, second_pass,
        "not a fixpoint: converting {html:?} gave {first_pass:?}, but reconverting its own \
         rendered HTML {rendered:?} gave {second_pass:?}"
    );
}

/// Spec example 278: a tight list whose non-first items are bare (`<p>`-less) fenced code
/// blocks. Any blank line our own rendering must add between such items (item boundaries would
/// otherwise be ambiguous) makes the WHOLE list loose on reparse, retroactively wrapping the
/// plain-text first item in `<p>` too -- so every item needs that same blank-line separation
/// from the first pass to match what the second pass produces.
#[test]
fn example_278_list_with_bare_code_block_items_reaches_fixpoint() {
    let html = "<ul>\n<li>foo</li>\n<li>\n<pre><code>bar\n</code></pre>\n</li>\n<li>\n<pre><code>baz\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(convert(html), "- foo\n\n- ```\n  bar\n  ```\n\n- ```\n  baz\n  ```\n");
    assert_reaches_fixpoint(html);
}

/// Spec example 299: three single-child lists nested inside each other with nothing else in
/// any enclosing `<li>`. Each inner list renders directly after its parent's bare marker on
/// the SAME physical line; a nested item starting fresh only needs its own indent when it is
/// actually starting a new line, not when continuing right after that marker.
#[test]
fn example_299_deeply_nested_single_child_lists_reaches_fixpoint() {
    let html = "<ol>\n<li>\n<ul>\n<li>\n<ol start=\"2\">\n<li>foo</li>\n</ol>\n</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(convert(html), "1. - 2. foo\n");
    assert_reaches_fixpoint(html);
}

/// Spec example 307: a fenced code block as the sole content of a THIRD-level nested list item
/// (bullet cycles to "+ " by then). A paragraph or bare marker check that only recognizes '*'
/// and '-' treats the "+ " marker as ordinary text, stacking a redundant continuation indent
/// onto the very first line of content and pushing it past the 4-space indented-code
/// threshold on reparse.
#[test]
fn example_307_third_level_bullet_paragraph_reaches_fixpoint() {
    let html =
        "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>\n<p>baz</p>\n<p>bim</p>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(convert(html), "- foo\n  * bar\n    + baz\n\n      bim\n");
    assert_reaches_fixpoint(html);
}

/// Spec example 308: two adjacent `<ul>` elements separated only by an HTML comment. `CommonMark`
/// treats a raw comment as the one thing that keeps two same-type lists from merging into one on
/// reparse, so that specific comment must be preserved instead of dropped like every other one.
#[test]
fn example_308_comment_separated_adjacent_lists_reaches_fixpoint() {
    let html = "<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<!-- -->\n<ul>\n<li>baz</li>\n<li>bim</li>\n</ul>\n";
    assert_eq!(convert(html), "- foo\n- bar\n\n<!-- -->\n\n- baz\n- bim\n");
    assert_reaches_fixpoint(html);
}

/// A comment between two adjacent lists of DIFFERENT types (`ul` then `ol`) is not the
/// merge-ambiguity separator `CommonMark` assigns comment meaning to here (different list types
/// never merge in the first place), so it keeps the ordinary drop-all-comments behavior.
#[test]
fn comment_between_different_list_types_is_still_dropped() {
    let html = "<ul>\n<li>foo</li>\n</ul>\n<!-- -->\n<ol>\n<li>bar</li>\n</ol>\n";
    let result = convert(html);
    assert!(
        !result.contains("<!--"),
        "comment between a ul and an ol should not be preserved: {result:?}"
    );
}

/// Spec example 319: a loose nested sublist (real `<p>` siblings) is the tail content of the
/// outer list's non-last item. Its own trailing blank line becomes the boundary before the
/// outer list's next item, which makes a `CommonMark`-compliant reparse treat the WHOLE outer
/// list as loose too -- including wrapping the outer item's own bare leading text in `<p>`.
#[test]
fn example_319_loose_nested_sublist_propagates_looseness_to_outer_item() {
    let html = "<ul>\n<li>a\n<ul>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n</ul>\n</li>\n<li>d</li>\n</ul>\n";
    assert_eq!(convert(html), "- a\n\n  * b\n\n    c\n\n- d\n");
    assert_reaches_fixpoint(html);
}

/// A loose nested sublist that is the ONLY item of its outer list must NOT propagate looseness
/// upward: there is no following outer sibling item for a boundary blank line to separate it
/// from, so forcing one here would add a spurious blank line with no purpose.
#[test]
fn loose_nested_sublist_as_sole_outer_item_does_not_force_leading_blank_line() {
    let html = "<ul><li>a<ul><li><p>b</p><p>c</p></li></ul></li></ul>";
    let result = convert(html);
    assert_eq!(result, "- a\n  * b\n\n    c\n");
}

/// Spec example 320: a blockquote directly follows an outer list item's bare leading text (no
/// `<p>`). A blockquote legally interrupts a preceding paragraph per `CommonMark` with no blank
/// line required, so forcing one here would make the leading text reparse as its own `<p>`,
/// flipping the whole list loose and desyncing the next pass from this one.
#[test]
fn example_320_blockquote_directly_after_bare_list_item_text_reaches_fixpoint() {
    let html = "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n</li>\n<li>c</li>\n</ul>\n";
    assert_eq!(convert(html), "- a\n  > b\n\n- c\n");
    assert_reaches_fixpoint(html);
}

/// Spec example 321: same as 320, plus a fenced code block continuing the same item after the
/// blockquote.
#[test]
fn example_321_blockquote_then_code_block_after_bare_list_item_text_reaches_fixpoint() {
    let html =
        "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n<pre><code>c\n</code></pre>\n</li>\n<li>d</li>\n</ul>\n";
    assert_eq!(convert(html), "- a\n  > b\n\n  ```\n  c\n  ```\n\n- d\n");
    assert_reaches_fixpoint(html);
}

/// A `<table>` as the LAST item's sole content must not retroactively force a leading blank
/// line onto an earlier, plain-text item: unlike a mid-list trigger, a block-forcing element
/// confined to the list's own last item has no following sibling to create a boundary blank
/// line with, so the list never actually reparses as loose.
#[test]
fn table_as_only_last_item_content_does_not_force_earlier_items_loose() {
    let html = "<ol><li><h3>h3</h3></li><li><table><caption>table</caption><tr><td>blah</td></tr></table></li></ol>";
    let result = convert(html);
    assert!(
        !result.contains("h3\n\n2."),
        "a trailing table-only item should not force a blank line before it: {result:?}"
    );
}
