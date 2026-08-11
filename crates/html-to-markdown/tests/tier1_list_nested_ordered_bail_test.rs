//! Tier-1 bails on nested lists where the current or an ancestor list is `<ol>`.
//!
//! Tier-1's `push_list_item_indent` hardcodes a uniform 2-space-per-depth scheme, which is
//! only correct when every ancestor list is unordered (`"- "` happens to be 2 columns wide,
//! matching the router's `list_indent_width == 2` gate). Tier-2 computes the cumulative width
//! of every ancestor marker instead, so it diverges from Tier-1 the moment an ordered list is
//! anywhere in a nested list's ancestor chain. `BailReason::ListNestedOrdered` routes those
//! cases to Tier-2, mirroring the precedent already set by the `list_indent_width != 2` gate
//! in `tier1/router.rs`.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1_run(html: &str) -> Result<String, BailReason> {
    let (cleaned, report) = prescan::run(html);
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    tier1::run(cleaned.as_ref(), &report, &opts)
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// `convert()` with `Tier1` forced — bails silently and falls back to Tier-2.
fn force_tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

#[test]
fn should_bail_when_ol_is_nested_in_ol() {
    let html = "<ol><li>outer<ol><li>inner</li></ol></li></ol>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::ListNestedOrdered),
        "expected ListNestedOrdered, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "Tier-1 fallback must match Tier-2");
}

#[test]
fn should_bail_when_ol_is_nested_in_ul() {
    let html = "<ul><li>outer<ol><li>inner</li></ol></li></ul>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::ListNestedOrdered),
        "expected ListNestedOrdered, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "Tier-1 fallback must match Tier-2");
}

#[test]
fn should_bail_when_ul_is_nested_in_ol() {
    let html = "<ol><li>outer<ul><li>inner</li></ul></li></ol>";
    let err = tier1_run(html).unwrap_err();
    assert!(
        matches!(err, BailReason::ListNestedOrdered),
        "expected ListNestedOrdered, got {err:?}"
    );
    assert_eq!(force_tier1(html), tier2(html), "Tier-1 fallback must match Tier-2");
}

#[test]
fn should_not_bail_when_ul_is_nested_in_ul() {
    // ~keep Every ancestor marker is "- " (2 columns), matching the uniform 2-space-per-depth
    // ~keep scheme Tier-1 hardcodes, so ul-in-ul nesting is handled natively.
    let html = "<ul><li>outer<ul><li>inner</li></ul></li></ul>";
    tier1_run(html).expect("Tier-1 should not bail on ul-in-ul nesting");
    assert_eq!(force_tier1(html), tier2(html));
}

#[test]
fn should_not_bail_on_top_level_ordered_list() {
    // ~keep A single, non-nested <ol> has no ancestor list, so the cumulative-width
    // ~keep concern does not apply.
    let html = "<ol><li>First</li><li>Second</li></ol>";
    tier1_run(html).expect("Tier-1 should not bail on a non-nested ordered list");
    assert_eq!(force_tier1(html), tier2(html));
}
