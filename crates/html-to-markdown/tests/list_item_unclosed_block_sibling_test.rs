//! Regression tests: an unclosed `<p>` or `<div>` inside a `<li>` must not merge or nest the
//! following sibling `<li>`.
//!
//! HTML5's parsing algorithm implies a closing `</li>` when a new `<li>` starts, regardless of
//! what block elements (`<p>`, `<div>`, …) are still open inside it. The `tl` parser used by
//! Tier-2 does not apply that rule, so `<li><div>content<li>next` (no explicit `</li>`, `</div>`)
//! nested the next `<li>` as a *child* of the `<div>` instead of treating it as a sibling list
//! item — producing `- - next` (an unwanted nested list) for `<div>` and `- x- next` (the second
//! item collapsed into literal text on the same line) for `<p>`.
//!
//! `has_inline_block_misnest` now also detects a `<li>`/`<dt>`/`<dd>` nested under another one
//! with no intervening `<ul>`/`<ol>`/`<dl>`, which routes the input through the `html5ever`
//! repair path (already used for other `tl` misparses) so the two items land as correct
//! siblings in both tiers.
//!
//! Corresponds to `CommonMark` spec example 175 ("HTML blocks").

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn auto(html: &str) -> String {
    convert(html, None).unwrap().content.unwrap_or_default()
}

/// Unclosed `<div>` inside a list item: the following `<li>` must remain a sibling, not a
/// nested list inside the first item.
#[test]
fn unclosed_div_in_list_item_keeps_next_li_a_sibling() {
    let html = "<ul><li><div></li><li>foo</li></ul>";
    let expected = "-\n\n- foo\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}

/// Unclosed `<p>` inside a list item: the second item must not be swallowed into the first
/// item's text on the same line.
#[test]
fn unclosed_p_in_list_item_keeps_next_li_a_sibling() {
    let html = "<ul><li><p>x</li><li>foo</li></ul>";
    let expected = "- x\n\n- foo\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}

/// Explicitly-closed `<div>` control case: already correct on `main`, must not regress.
#[test]
fn closed_div_in_list_item_keeps_next_li_a_sibling() {
    let html = "<ul><li><div></div></li><li>foo</li></ul>";
    let expected = "-\n\n- foo\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}

/// Unclosed `<span>` (inline) control case: already correct on `main` via the pre-existing
/// block-under-inline misnest detection, must not regress.
#[test]
fn unclosed_span_in_list_item_keeps_next_li_a_sibling() {
    let html = "<ul><li><span>x</li><li>foo</li></ul>";
    let expected = "- x\n- foo\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}

/// A chain of three unclosed `<div>`-wrapped items must all land as siblings, not a
/// three-deep nested chain.
#[test]
fn chain_of_unclosed_divs_keeps_all_li_siblings() {
    let html = "<ul><li><div>a<li><div>b<li><div>c</li></ul>";
    let expected = "- a\n\n- b\n\n- c\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}

/// A genuinely nested list (`<li>` inside a `<ul>` inside another `<li>`, with the outer
/// item's `<p>` properly closed) must not be misdetected as a misnest and must keep its
/// nested-list rendering.
#[test]
fn genuinely_nested_list_after_closed_p_is_unaffected() {
    let html = "<ul><li><p>outer</p><ul><li>inner</li></ul></li></ul>";
    let expected = "- outer\n\n  * inner\n";
    assert_eq!(auto(html), expected, "auto tier mismatch for: {html:?}");
    assert_eq!(tier2(html), expected, "tier2 mismatch for: {html:?}");
    assert_eq!(tier1(html), expected, "tier1 mismatch for: {html:?}");
}
