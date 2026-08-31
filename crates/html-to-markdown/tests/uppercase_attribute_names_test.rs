//! HTML attribute names are case-insensitive, but the Tier-2 converter matched them
//! byte-for-byte as written until the `astral-tl` 0.8.0 parser upgrade, which lowercases
//! attribute keys at parse time. Before it, `<a HREF="x">text</a>` converted to bare
//! `text` with the destination silently dropped, and `<img SRC=... ALT=...>` converted to
//! an empty `![](<>)` -- content loss, not just a formatting difference.
//!
//! Tier-1's `find_attr`/`has_attr` have always compared names with `eq_ignore_ascii_case`,
//! so every case here is also asserted to agree across both tiers: the library picks a
//! tier automatically, and these inputs are exactly the shape that made one document
//! convert two ways.

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

fn assert_both_tiers(html: &str, expected: &str) {
    let t2 = run_tier2(html);
    assert_eq!(t2, expected, "tier2 output changed for {html:?}");
    let t1 = run_tier1(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

#[test]
fn uppercase_href_keeps_the_link_destination() {
    assert_both_tiers("<a HREF=\"up.html\">link</a>", "[link](up.html)\n");
}

#[test]
fn uppercase_src_and_alt_keep_the_image() {
    assert_both_tiers("<img SRC=\"a.png\" ALT=\"cat\">", "![cat](a.png)\n");
}

#[test]
fn mixed_case_attribute_names_are_matched() {
    assert_both_tiers("<a HrEf=\"m.html\">link</a>", "[link](m.html)\n");
}

#[test]
fn lowercase_attribute_names_are_unaffected() {
    assert_both_tiers("<a href=\"low.html\">link</a>", "[link](low.html)\n");
}

#[test]
fn a_repeated_attribute_takes_its_first_value() {
    // ~keep HTML5 tree construction drops later duplicates of an attribute already set.
    assert_both_tiers(
        "<a href=\"first.html\" href=\"second.html\">link</a>",
        "[link](first.html)\n",
    );
}
