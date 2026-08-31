//! Regression tests for a Tier-1 defect where content inside `<table>` but outside any
//! row/cell/caption -- HTML5's "foster parented" position -- was silently dropped
//! instead of surviving (as Tier-2 does) or reliably bailing to the Tier-2 fallback.
//!
//! Two independent root causes, both in `src/converter/tier1/scanner.rs`:
//!
//! 1. `emit_close_for_implicit`'s `TagKind::Table` arm was a no-op. HTML5 parsers
//!    implicitly close every still-open element at EOF (see the Phase N2 comment in
//!    `scan()`), so an unclosed `<table>` -- e.g. `<table><tr><td>x</td></tr>` with no
//!    trailing `</table>` -- hit that no-op arm instead of `close_table`, discarding the
//!    ENTIRE accumulated table (fully-formed rows included), where the explicit
//!    `</table>` path already called `close_table` correctly.
//! 2. `flush_text`'s "text directly inside `<table>`, outside any cell/caption" branch
//!    unconditionally discarded the text. That is correct for insignificant whitespace
//!    between structural tags, but not for real, non-whitespace text: Tier-2's `tl`-based
//!    DOM sometimes resurfaces such text as real pre-table content (foster-parented,
//!    matching the real HTML5 tree-construction algorithm), depending on adjacency to a
//!    `<!--comment-->` in a way a single-pass byte scanner cannot cheaply reproduce.
//!    Tier-1 now bails on any non-whitespace occurrence instead of guessing.

#![cfg(feature = "testkit")]

mod support;

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

/// Run `tier1::run` directly (no fallback) so a bail is visible as `Err`, not silently
/// swallowed by `convert()`'s production fallback-to-Tier-2 path.
fn tier1_run(html: &str) -> Result<String, BailReason> {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(html, &PrescanReport::default(), &opts)
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// `convert()` forced onto the Tier-1 testkit path, which still falls back to Tier-2 on
/// bail -- this is what a real caller observes, independent of which tier internally
/// produced the byte-identical result.
fn tier1_via_convert(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Assert that, whatever Tier-1 internally does (render or bail), the value a caller of
/// `convert()` actually observes is byte-identical to Tier-2's.
fn assert_matches_tier2(html: &str) {
    let observed = tier1_via_convert(html);
    let t2 = tier2(html);
    assert_eq!(
        observed, t2,
        "tier1-forced convert() diverged from tier2\ninput: {html:?}\nobserved: {observed:?}\ntier2: {t2:?}"
    );
}

// ---- Root cause 1: unclosed `<table>` at EOF must not drop already-formed rows ----

/// The core regression: two fully-formed rows survive an unclosed `<table>`. Before the
/// fix, `emit_close_for_implicit`'s no-op `TagKind::Table` arm dropped both rows.
#[test]
fn unclosed_table_with_rows_renders_via_tier1_directly() {
    let html = "<table><tr><td>x</td></tr><tr><td>y</td></tr>";
    let t1 = tier1_run(html).expect("a table with real rows must render, not bail");
    assert_eq!(t1, tier2(html));
    assert_eq!(t1, "| x |\n| --- |\n| y |\n");
}

/// A single unclosed row likewise survives.
#[test]
fn unclosed_table_single_row() {
    assert_matches_tier2("<table><tr><td>x</td></tr>");
}

/// An unclosed table with no rows at all is `is_blank` in `close_table`'s own existing
/// safety check, which bails rather than emitting -- exercised here through the
/// EOF/implicit-close path specifically (not the explicit `</table>` path that already
/// exercised it before this fix).
#[test]
fn unclosed_blank_table_bails_and_falls_back() {
    let err = tier1_run("<table>").expect_err("a rowless table must bail, not emit empty");
    assert!(matches!(err, BailReason::Classifier), "unexpected bail reason: {err:?}");
    assert_matches_tier2("<table>");
}

// ---- Root cause 2: non-whitespace text outside any cell/caption must not vanish ----

/// Whitespace-only text between structural tags is unaffected -- still silently
/// dropped, matching Tier-2, no bail.
#[test]
fn whitespace_only_text_between_structural_tags_still_silently_dropped() {
    let html = "<table>   <tr><td>x</td></tr></table>";
    let t1 = tier1_run(html).expect("whitespace-only stray text must not bail");
    assert_eq!(t1, tier2(html));
}

/// Text before any row, properly closed, with a comment -- the exact family the
/// `table-foster-parenting-text-comment.html` fixture belongs to. Tier-2 resurfaces the
/// text as real pre-table content; Tier-1 bails reliably and the fallback matches.
#[test]
fn text_before_any_row_with_comment_bails_and_falls_back() {
    let html = "<table>abc<!--c--></table>";
    let err = tier1_run(html).expect_err("foster-parented text must bail, not drop silently");
    assert!(matches!(err, BailReason::Classifier), "unexpected bail reason: {err:?}");
    assert_matches_tier2(html);
}

/// Same shape without a comment: Tier-2 also drops the text here (no comment-adjacency
/// quirk to resurface it), so this is a case where Tier-1's bail is strictly more
/// conservative than necessary -- but the fallback still matches byte-for-byte.
#[test]
fn text_before_any_row_no_comment_bails_and_falls_back() {
    assert_matches_tier2("<table>abc</table>");
}

/// Text before any row, comment, AND a following row -- confirms the bail fires even
/// when the table is otherwise perfectly well-formed and would render fine on its own.
#[test]
fn text_before_row_with_comment_and_following_row_bails_and_falls_back() {
    assert_matches_tier2("<table>abc<!--c--><tr><td>x</td></tr></table>");
}

/// Text between two rows, with a comment: Tier-2 hoists it entirely before the table.
#[test]
fn text_between_rows_with_comment_bails_and_falls_back() {
    let html = "<table><tr><td>x</td></tr>mid<!--c--><tr><td>y</td></tr></table>";
    let err = tier1_run(html).expect_err("foster-parented text between rows must bail");
    assert!(matches!(err, BailReason::Classifier), "unexpected bail reason: {err:?}");
    assert_matches_tier2(html);
}

/// Text between two rows, no comment: Tier-2 also drops it, but Tier-1 still bails
/// (same conservative-but-correct-via-fallback behaviour as the before-any-row case).
#[test]
fn text_between_rows_no_comment_bails_and_falls_back() {
    assert_matches_tier2("<table><tr><td>x</td></tr>mid<tr><td>y</td></tr></table>");
}

/// Text after the last row, with a comment: Tier-2 hoists it before the whole table.
#[test]
fn text_after_last_row_with_comment_bails_and_falls_back() {
    let html = "<table><tr><td>x</td></tr>after<!--c--></table>";
    let err = tier1_run(html).expect_err("foster-parented trailing text must bail");
    assert!(matches!(err, BailReason::Classifier), "unexpected bail reason: {err:?}");
    assert_matches_tier2(html);
}

/// Text after the last row, no comment: Tier-2 drops it; Tier-1's fallback still matches.
#[test]
fn text_after_last_row_no_comment_bails_and_falls_back() {
    assert_matches_tier2("<table><tr><td>x</td></tr>after</table>");
}

/// The unclosed-at-EOF variant of "text after the last row, with comment" -- both root
/// causes could plausibly interact here (EOF implicit-close AND foster-parented text);
/// confirm the fallback still matches.
#[test]
fn text_after_last_row_with_comment_unclosed_bails_and_falls_back() {
    assert_matches_tier2("<table><tr><td>x</td></tr>after<!--c-->");
}

// ---- Real-world fixtures from `test_documents/html/html5lib/` ----

fn read_fixture(name: &str) -> String {
    let path = support::corpus_root().join("html5lib").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// The exact reproducer named in the defect report: an unclosed `<table>` whose only
/// content is foster-parented text followed by a comment.
#[test]
fn fixture_table_foster_parenting_text_comment() {
    let html = read_fixture("table-foster-parenting-text-comment.html");
    let err = tier1_run(&html).expect_err("fixture's foster-parented text must bail, not drop silently");
    assert!(matches!(err, BailReason::Classifier), "unexpected bail reason: {err:?}");
    let observed = tier1_via_convert(&html);
    let t2 = tier2(&html);
    assert_eq!(observed, t2);
    assert_eq!(
        t2, "abc\n",
        "tier2 ground truth changed; update this test's expectation"
    );
    assert!(
        !observed.is_empty(),
        "regression: foster-parented text dropped entirely"
    );
}

/// A stray `<td>` directly under `<table>` with no `<tr>` wrapper at all (HTML5 inserts
/// an implicit `<tr>`), with foster-parented text on both sides of the cell.
#[test]
fn fixture_table_foster_parenting_text_no_tr() {
    let html = read_fixture("table-foster-parenting-text.html");
    assert_matches_tier2(&html);
}

/// A stray `<a>` then `<td>` directly under `<table>`, no `<tr>` wrapper.
#[test]
fn fixture_table_foster_parenting_anchor_td() {
    let html = read_fixture("table-foster-parenting-anchor-td.html");
    assert_matches_tier2(&html);
}
