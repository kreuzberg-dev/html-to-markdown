//! Tier-1 list-item layout (Phase EE).
//!
//! Tier-2's `handle_li` handles two layout shapes that Tier-1 used to
//! get wrong:
//!   * `<li><p>text</p></li>` — the `<p>` is the first block child of
//!     the list item, so the text follows the bullet inline (`- text`).
//!     Tier-1 used to emit `- \n\nText` (bullet on its own line).
//!   * `<li><p>foo</p><pre>bar</pre></li><li>baz</li>` — loose list:
//!     because the first item has block children, the next bullet
//!     starts after a blank line.  Tier-1 used to emit them tight.
//!   * Multi-line text inside `<code>` was being whitespace-collapsed;
//!     Tier-2 preserves it verbatim like `<pre>`.

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

fn assert_matches(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn paragraph_inside_list_item_inline() {
    assert_matches("<ul><li><p>x</p></li></ul>");
}

#[test]
fn paragraph_inside_ordered_list_item_inline() {
    assert_matches("<ol><li><p>x</p></li></ol>");
}

#[test]
fn loose_list_after_block_child_emits_blank_line() {
    assert_matches("<ul><li><p>a</p><pre>code</pre></li><li>b</li></ul>");
}

#[test]
fn tight_list_no_blank_line() {
    assert_matches("<ul><li>a</li><li>b</li></ul>");
}

#[test]
fn code_preserves_whitespace_runs_verbatim() {
    assert_matches("<p><code>a   b</code></p>");
}

#[test]
fn code_preserves_newlines_verbatim() {
    assert_matches("<p><code>line1\n   line2</code></p>");
}

#[test]
fn nested_list_pre_continuation_indent() {
    assert_matches("<ul><li><ul><li><p>a</p><pre>code</pre></li></ul></li></ul>");
}

#[test]
fn ordered_list_pre_continuation_indent_matches_marker_width() {
    assert_matches("<ol><li><p>a</p><pre>code</pre></li><li>b</li></ol>");
}

#[test]
fn ordered_list_pre_continuation_indent_widens_for_double_digit_marker() {
    assert_matches("<ol start=\"9\"><li>a</li><li>b<br>c</li></ol>");
}

#[test]
fn blockquote_continuation_indent_multiline() {
    assert_matches("<ul><li><p>a</p><blockquote>b\nc</blockquote></li></ul>");
}

/// A nested list directly following a `<strong>` that ends in whitespace must get its own
/// line in both tiers -- the closing `"**"` plus the wrapper's migrated trailing space is
/// literally the same two bytes as a bare `"* "` bullet marker, which used to make both
/// tiers' bare-marker check false-positive and flatten the nested list onto the parent line.
#[test]
fn nested_list_after_strong_with_trailing_space_matches() {
    assert_matches("<ul><li><strong>b </strong><ul><li>sub</li></ul></li></ul>");
}

/// Same trigger with a real-world Unicode whitespace character (EN SPACE, U+2002) -- the
/// shape that surfaced this in the generated fixture corpus
/// (`generated:seed=15118233204572906709`).
#[test]
fn nested_list_after_strong_with_trailing_unicode_space_matches() {
    assert_matches("<ul><li><strong>b\u{2002}</strong><ul><li>sub</li></ul></li></ul>");
}

/// Same trigger via `<em>`.
#[test]
fn nested_list_after_emphasis_with_trailing_space_matches() {
    assert_matches("<ul><li><em>b </em><ul><li>sub</li></ul></li></ul>");
}

/// Three single-child unordered lists nested directly inside each other stack their bare
/// markers on one physical line with nothing else between them. All-unordered nesting is
/// handled natively by Tier-1 (no ordered-list bail), so this exercises Tier-1's own
/// bare-marker recursion rather than its Tier-2 fallback path.
#[test]
fn deeply_nested_single_child_unordered_lists_matches() {
    assert_matches("<ul><li><ul><li><ul><li>x</li></ul></li></ul></li></ul>");
}

#[test]
fn text_after_br_gets_list_continuation_indent() {
    assert_matches("<ul><li>a<br>b</li></ul>");
}
