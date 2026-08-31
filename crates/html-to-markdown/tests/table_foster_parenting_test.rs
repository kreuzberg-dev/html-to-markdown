//! Regression tests for a Tier-2 defect: text (or an element outside the small set valid
//! directly under `<table>`) placed as a direct child of `<table>` -- outside any
//! row/cell/caption -- is HTML5's "foster parented" position. A spec-compliant parser
//! relocates that content to just before the `<table>` (text/disallowed elements) or
//! restructures it in place (a stray `<td>` gets an implicit `<tr>`); it survives either
//! way. `tl` has no foster-parenting or table auto-fixup and leaves such content exactly
//! where it was written, so the table handler -- which only recognises
//! `caption`/`thead`/`tbody`/`tfoot`/`tr`/`colgroup`/`col` as direct children of `<table>`
//! -- silently drops raw text and routes anything else through a no-op handler.
//!
//! The fix extends `has_inline_block_misnest` (`src/converter/preprocessing_helpers.rs`)
//! to detect a `<table>` with such a direct child and route the document through the
//! existing `repair_with_html5ever` escape hatch, which implements the HTML5 tree
//! construction algorithm's "in table" insertion mode -- including foster parenting --
//! correctly. `html5ever`'s repaired structure is used below as the ground-truth oracle
//! for every expected string.

#![cfg(feature = "testkit")]

mod support;

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

// ---- Text directly inside `<table>`, no comment involved ----

/// Text before any row: foster-parented in front of the whole table.
#[test]
fn text_before_any_row_survives() {
    assert_eq!(
        tier2("<table>abc<tr><td>x</td></tr></table>"),
        "abc\n\n| x |\n| --- |\n"
    );
}

/// Text between two rows: also foster-parented in front of the whole table, not left
/// between the rendered rows -- HTML5 foster parenting always targets the position
/// immediately before the table, regardless of when the stray text was encountered.
#[test]
fn text_between_rows_survives() {
    assert_eq!(
        tier2("<table><tr><td>x</td></tr>between<tr><td>y</td></tr></table>"),
        "between\n\n| x |\n| --- |\n| y |\n"
    );
}

/// Text after the last row: likewise hoisted before the table.
#[test]
fn text_after_last_row_survives() {
    assert_eq!(
        tier2("<table>abc<tr><td>x</td></tr>after</table>"),
        "abcafter\n\n| x |\n| --- |\n"
    );
}

/// A table whose only content is stray text, with a following sibling paragraph: the
/// original defect report's minimal repro.
#[test]
fn table_with_only_stray_text_and_no_rows() {
    assert_eq!(tier2("<table>abc</table>"), "abc\n");
}

/// Confirms the fix removes the previous asymmetry: whether or not a trailing comment
/// is present no longer changes whether the text survives.
#[test]
fn table_with_only_stray_text_and_trailing_comment_matches_no_comment_case() {
    assert_eq!(tier2("<table>abc</table>"), tier2("<table>abc<!--c--></table>"));
    assert_eq!(tier2("<table>abc<!--c--></table>"), "abc\n");
}

/// Text preceding a table that has real rows and semantic content, with an unrelated
/// preceding paragraph -- checks the hoisted text lands after the paragraph and before
/// the table, not swallowed into either.
#[test]
fn text_before_row_after_a_preceding_paragraph() {
    assert_eq!(
        tier2("<p>before</p><table>abc<tr><td>x</td></tr></table>"),
        "before\n\nabc\n\n| x |\n| --- |\n"
    );
}

// ---- Same shapes, unclosed `<table>` ----

/// Text before a single row, with the table never explicitly closed.
#[test]
fn text_before_row_unclosed_table_survives() {
    assert_eq!(tier2("<table>abc<tr><td>x</td></tr>"), "abc\n\n| x |\n| --- |\n");
}

/// A rowless, unclosed table whose only content is stray text.
#[test]
fn stray_text_only_unclosed_table_survives() {
    assert_eq!(tier2("<table>abc"), "abc\n");
}

// ---- Whitespace-only stray text must NOT be treated as foster-parenting candidate ----

/// Whitespace-only text directly under `<table>` is inserted normally by HTML5 (it
/// stays inside the table) and carries no content worth surfacing -- so it must not
/// trigger a repair pass, and the existing (harmless) drop behaviour is unchanged.
#[test]
fn whitespace_only_stray_text_does_not_trigger_repair() {
    assert_eq!(tier2("<table>   <tr><td>x</td></tr></table>"), "| x |\n| --- |\n");
}

/// A table containing only whitespace and no rows renders as nothing, same as an
/// empty table.
#[test]
fn whitespace_only_table_with_no_rows_renders_empty() {
    assert_eq!(tier2("<table> </table>"), "");
    assert_eq!(tier2("<table></table>"), "");
}

// ---- A stray `<td>` or `<a>` directly under `<table>`, no `<tr>` wrapper ----

/// A lone `<td>` with no `<tr>` wrapper: HTML5 does not foster-parent this -- it
/// implicitly inserts the missing `<tr>` (and `<tbody>`) around it, so it renders as a
/// normal one-cell table instead of vanishing into the no-op handler for a stray `<td>`.
#[test]
fn stray_td_with_no_tr_gets_implicit_row() {
    assert_eq!(tier2("<table><td>x</td></table>"), "| x |\n| --- |\n");
}

/// A stray `<a>` directly under `<table>`: HTML5 foster-parents the whole element (not
/// just its text) to just before the table.
#[test]
fn stray_anchor_with_no_tr_is_hoisted() {
    assert_eq!(tier2(r##"<table><a href="#">x</a></table>"##), "[x](#)\n");
}

/// Text and a stray `<td>` (no `<tr>`) both directly under `<table>`: the surrounding
/// text is foster-parented before the table while the cell gets its implicit row --
/// mirrors the `table-foster-parenting-text.html` html5lib fixture already pinned in
/// `tier1_table_foster_parenting_test.rs`.
#[test]
fn stray_td_between_foster_parented_text() {
    assert_eq!(tier2("<table>A<td>B</td>C</table>"), "AC\n\n| B |\n| --- |\n");
}

// ---- Real-world fixtures from `test_documents/html/html5lib/` ----

fn read_fixture(name: &str) -> String {
    let path = support::corpus_root().join("html5lib").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

#[test]
fn fixture_table_foster_parenting_text_comment() {
    let html = read_fixture("table-foster-parenting-text-comment.html");
    assert_eq!(tier2(&html), "abc\n");
}

#[test]
fn fixture_table_foster_parenting_text_no_tr() {
    let html = read_fixture("table-foster-parenting-text.html");
    assert_eq!(tier2(&html), "AC\n\n| B |\n| --- |\n");
}

#[test]
fn fixture_table_foster_parenting_anchor_td() {
    let html = read_fixture("table-foster-parenting-anchor-td.html");
    assert_eq!(tier2(&html), "13\n\n| 2 |\n| --- |\n");
}
