//! Regression tests for Tier-1 scanner panic hardening (TIER1-16).
//!
//! The Tier-1 scanner tracks byte offsets into its output buffer (notably
//! `OpenTag::content_start`) that are captured when a tag opens and read back
//! when it closes, and offsets into the input (`find_balanced_close` and
//! friends) that are captured mid-scan. These sites are not reachable through
//! the default public API today — `classify()` currently routes every input
//! to Tier-2 (see the `highlight_style` gate in `router.rs`, deliberately left
//! in place pending a separate unblocking task) — so they were latent even
//! under 500k-case fuzzing of `TierStrategy::Auto`. Once that gate lifts they
//! become live, so this suite exercises the scanner directly via
//! `TierStrategy::Tier1` (testkit-only) and asserts it never panics,
//! regardless of whether it produces output or bails.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// Force the Tier-1 path (bypassing the classifier) and assert conversion
/// never panics, whether it succeeds or bails back to Tier-2.
fn assert_tier1_does_not_panic(html: &str) {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let html_owned = html.to_owned();
    let result = std::panic::catch_unwind(move || convert(&html_owned, Some(opts)));
    assert!(result.is_ok(), "Tier-1 forced conversion panicked on input: {html:?}");
}

// ~keep ── `find_balanced_close` off-by-one (unterminated SVG close tag) ────────────
// ~keep Before the fix, a `<svg>`/`</svg` pair whose closing tag had no terminating
// ~keep `>` before EOF caused `find_balanced_close` to return `Some(bytes.len() + 1)`
// ~keep instead of `None`, which the caller then sliced `&html[start..len + 1]` —
// ~keep an out-of-bounds panic.

#[test]
fn svg_unterminated_close_tag_does_not_panic() {
    assert_tier1_does_not_panic("<p>hi</p><svg></svg");
}

#[test]
fn svg_nested_unterminated_close_tag_does_not_panic() {
    assert_tier1_does_not_panic("<p>hi</p><svg><svg></svg></svg");
}

#[test]
fn svg_unterminated_close_tag_at_document_start_does_not_panic() {
    assert_tier1_does_not_panic("<svg></svg");
}

// ~keep ── Stale `content_start` (open/close frame offset) hardening ────────────────
// ~keep `OpenTag::content_start` is captured as `buf.len()` when a tag opens and
// ~keep read back at close time to slice/truncate/insert into that same buffer.
// ~keep These cases stress deeply nested and empty inline/block elements — the
// ~keep shapes most likely to desync the recorded offset from the buffer it is
// ~keep later applied to (e.g. a close handler routed to the wrong buffer, or an
// ~keep inner close mutating content an outer frame's offset assumed was fixed).

#[test]
fn deeply_nested_empty_inline_elements_do_not_panic() {
    let html = "<p>".to_string() + &"<strong><em>".repeat(200) + &"</em></strong>".repeat(200) + "</p>";
    assert_tier1_does_not_panic(&html);
}

#[test]
fn deeply_nested_whitespace_only_inline_elements_do_not_panic() {
    let html = "<p>".to_string() + &"<strong>   <em>  ".repeat(200) + &"  </em>   </strong>".repeat(200) + "</p>";
    assert_tier1_does_not_panic(&html);
}

#[test]
fn empty_heading_with_multibyte_boundary_content_does_not_panic() {
    assert_tier1_does_not_panic("<h1>\u{00a0}\u{00a0}\u{00a0}   </h1>");
}

#[test]
fn empty_blockquote_and_pre_nested_do_not_panic() {
    assert_tier1_does_not_panic("<blockquote><pre></pre></blockquote>");
}

#[test]
fn empty_code_and_link_adjacent_do_not_panic() {
    assert_tier1_does_not_panic("<p><code></code><a href=\"#\"></a></p>");
}

#[test]
fn multibyte_content_around_inline_markers_does_not_panic() {
    // ~keep NBSP (2 bytes) and CJK (3 bytes) around close-marker boundaries.
    assert_tier1_does_not_panic("<p><strong>\u{00a0}\u{4e2d}\u{6587}\u{00a0}</strong></p>");
    assert_tier1_does_not_panic("<h2>\u{00a0}\u{4e2d}\u{6587}</h2>");
    assert_tier1_does_not_panic("<a href=\"#\">\u{00a0}\u{4e2d}\u{6587}\u{00a0}</a>");
}

#[test]
fn nested_list_items_with_empty_and_multibyte_content_do_not_panic() {
    let html = "<ul>".to_string() + &"<li><ul><li>\u{00a0}</li></ul></li>".repeat(50) + "</ul>";
    assert_tier1_does_not_panic(&html);
}
