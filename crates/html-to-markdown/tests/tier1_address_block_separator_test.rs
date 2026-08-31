//! Regression coverage for a Tier-1/Tier-2 divergence on `<address>` (and the other
//! `TagKind::Block` tag names that Tier-2's `main.rs` dispatch match does not give a
//! dedicated separator-emitting handler: `<search>`, `<hgroup>`, `<center>`,
//! `<colgroup>`, `<col>`, `<base>`, `<html>`, `<body>`).
//!
//! Tier-2 routes these through `block::unknown::handle` (or, for `<html>`/`<body>`,
//! `block::container::handle_structural_container`) -- both walk children with NO
//! leading or trailing `"\n\n"` block separator of their own, unlike `<div>` and the
//! semantic/media/form-dispatched block tags. Tier-1's generic `TagKind::Block` open/close
//! handling in `src/converter/tier1/scanner.rs` used to give every one of these names the
//! same separator treatment as `<div>`, which is wrong for this specific subset. See
//! `block_container_is_passthrough` in `scanner.rs` for the fix.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy, convert};

fn tier1_friendly_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

/// Runs the Tier-1 scanner directly (never through `convert()`'s fallback dispatch) so a
/// bail is a hard test failure, not a silent fall-through to Tier-2.
fn run_tier1(html: &str) -> String {
    let report = PrescanReport::default();
    match tier1::run(html, &report, &tier1_friendly_options()) {
        Ok(markdown) => markdown,
        Err(reason) => panic!("tier1 bailed on {html:?}: {reason:?}"),
    }
}

fn run_tier2(html: &str) -> String {
    let mut options = tier1_friendly_options();
    options.tier_strategy = TierStrategy::Tier2;
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

fn assert_tier1_matches_tier2(html: &str) {
    let t1 = run_tier1(html);
    let t2 = run_tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn adjacent_address_elements_have_no_separator() {
    let html = "<address>foo</address><address>bar</address>";
    assert_eq!(
        run_tier2(html),
        "foobar\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "foobar\n");
    assert_tier1_matches_tier2(html);
}

#[test]
fn address_between_real_block_siblings_still_gets_their_separators() {
    // ~keep The separator here comes from the SURROUNDING `<p>`/`<div>` handlers, not from
    // ~keep `<address>` itself -- this must keep working after the passthrough fix.
    assert_tier1_matches_tier2("<p>a</p><address>b</address><p>c</p>");
    assert_tier1_matches_tier2("<div>a</div><address>b</address><div>c</div>");
}

#[test]
fn lone_address_element_renders_plain() {
    assert_eq!(run_tier2("<address>only</address>"), "only\n");
    assert_tier1_matches_tier2("<address>only</address>");
}

#[test]
fn address_inside_table_cell_gets_no_separator_or_space() {
    let html = "<table><tr><td><address>a</address><address>b</address></td></tr></table>";
    assert_eq!(
        run_tier2(html),
        "| ab |\n| --- |\n",
        "tier2 ground truth changed; update this test"
    );
    assert_tier1_matches_tier2(html);
}

#[test]
fn search_hgroup_center_colgroup_also_have_no_separator() {
    for tag in ["search", "hgroup", "center", "colgroup"] {
        let html = format!("<{tag}>foo</{tag}><{tag}>bar</{tag}>");
        assert_tier1_matches_tier2(&html);
    }
}

#[test]
fn div_still_gets_its_own_separator_unaffected_by_the_fix() {
    assert_tier1_matches_tier2("<div>foo</div><div>bar</div>");
}
