//! Regression test for the Tier-1/Tier-2 GFM autolink divergence.
//!
//! `autolinks` defaults to `true` and is not gated in `tier1::router::classify`, so every
//! default-options `<a>` whose visible text equals its `href` must produce Tier-2's GFM
//! autolink form (`<href>`) on Tier-1 too, instead of the unconditional `[text](href)` form
//! Tier-1 emitted before this fix (`converter/handlers/link.rs:91-101` is the Tier-2
//! predicate this mirrors).
//!
//! Uses `tier1::run` directly (not `convert()` with `TierStrategy::Tier1`, which silently
//! falls back to Tier-2 on a bail) so a Tier-1 bail is a hard test failure.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::{self, PrescanReport};
use html_to_markdown_rs::tier1;
use html_to_markdown_rs::tier1::BailReason;
use html_to_markdown_rs::tier1::router::{RouterDecision, classify};
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1_friendly_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        ..ConversionOptions::default()
    }
}

/// Runs the Tier-1 scanner directly so a bail is a hard test failure, not a silent
/// fall-through to Tier-2.
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
fn link_text_matching_href_becomes_an_autolink() {
    assert_both_tiers(r#"<a href="https://x.com">https://x.com</a>"#, "<https://x.com>\n");
}

#[test]
fn link_text_matching_href_is_an_autolink_inside_a_paragraph() {
    assert_both_tiers(
        r#"<p>See <a href="https://x.com">https://x.com</a> for more.</p>"#,
        "See <https://x.com> for more.\n",
    );
}

#[test]
fn mailto_link_text_matching_the_address_becomes_an_autolink() {
    assert_both_tiers(r#"<a href="mailto:a@b.com">a@b.com</a>"#, "<a@b.com>\n");
}

#[test]
fn mailto_link_text_matching_the_full_href_becomes_an_autolink() {
    assert_both_tiers(r#"<a href="mailto:a@b.com">mailto:a@b.com</a>"#, "<mailto:a@b.com>\n");
}

#[test]
fn non_matching_link_text_stays_the_normal_link_form() {
    assert_both_tiers(
        r#"<a href="https://x.com">click here</a>"#,
        "[click here](https://x.com)\n",
    );
}

#[test]
fn bare_path_href_never_autolinks_even_when_text_matches() {
    // ~keep `has_uri_scheme` requires an absolute URI with a scheme; a bare relative
    // ~keep path must stay `[text](href)` even when text equals href verbatim.
    assert_both_tiers(r#"<a href="up.html">up.html</a>"#, "[up.html](up.html)\n");
}

#[test]
fn autolinks_disabled_keeps_the_normal_link_form() {
    let html = r#"<a href="https://x.com">https://x.com</a>"#;
    let mut options = tier1_friendly_options();
    options.autolinks = false;
    options.tier_strategy = TierStrategy::Tier2;
    let t2 = convert(html, Some(options.clone()))
        .unwrap()
        .content
        .unwrap_or_default();
    assert_eq!(t2, "[https://x.com](https://x.com)\n");

    let report = PrescanReport::default();
    let t1 = match tier1::run(html, &report, &options) {
        Ok(markdown) => markdown,
        Err(reason) => panic!("tier1 bailed on {html:?}: {reason:?}"),
    };
    assert_eq!(t1, t2, "tier1 diverged from tier2\ntier1: {t1:?}\ntier2: {t2:?}");
}

#[test]
fn autolink_drops_a_title_attribute_on_both_tiers() {
    // ~keep Tier-2's `is_autolink` predicate does not consider `title` at all
    // ~keep (`handlers/link.rs:91-101`), and its autolink branch returns before any
    // ~keep title is appended -- so an explicit `title=` is silently dropped. Tier-1's
    // ~keep autolink branch discards the popped title by returning early, which is only
    // ~keep correct because Tier-2 does the same; this pins that agreement.
    assert_both_tiers(
        r#"<a href="https://x.com" title="ignored">https://x.com</a>"#,
        "<https://x.com>\n",
    );
}

#[test]
fn default_title_is_rejected_by_the_router_so_tier1_never_sees_it() {
    // ~keep Tier-1's autolink branch tests `!options.default_title` but does not
    // ~keep implement the title-from-href behaviour. That is only safe because the
    // ~keep ROUTER rejects `default_title` before Tier-1 is ever reached -- note this is
    // ~keep a `classify()` gate, not a scanner bail: `tier1::run` is the raw scanner
    // ~keep entry point and does not consult the router, so asserting on it would prove
    // ~keep nothing. Pinning the gate here means removing it fails this test rather than
    // ~keep silently exposing the unimplemented path.
    let html = r#"<a href="https://x.com">https://x.com</a>"#;
    let mut options = tier1_friendly_options();
    options.default_title = true;

    let (_cleaned, report) = prescan::run(html);
    assert_eq!(
        classify(&report, &options),
        RouterDecision::Tier2,
        "router must keep default_title on Tier-2"
    );

    let mut tier2_options = options;
    tier2_options.tier_strategy = TierStrategy::Tier2;
    assert_eq!(
        convert(html, Some(tier2_options)).unwrap().content.unwrap_or_default(),
        "[https://x.com](https://x.com \"https://x.com\")\n"
    );
}

#[test]
fn nested_markup_label_equal_to_href_agrees_across_tiers() {
    // ~keep Tier-2 builds its autolink predicate from the TAG-STRIPPED text content
    // ~keep (`get_text_content`/`inline_label`), so `<b>` around the URL is invisible to
    // ~keep it and it still emits the bare autolink. Tier-1 compares the RENDERED label,
    // ~keep which is `**https://x.com**` and can never equal the href. Tier-1 must
    // ~keep therefore bail on this shape rather than emitting a different link form --
    // ~keep returning `Ok` with `[**url**](url)` here would be exactly the
    // ~keep `Ok(wrong_output)` that `tier1_property_test.rs` forbids. It must bail with
    // ~keep `LinkAutolinkNestedMarkup` specifically, not merely bail for some unrelated
    // ~keep reason -- pin the reason so a future refactor that starts bailing earlier
    // ~keep (e.g. `DepthMismatch`) for the wrong cause does not silently mask this fix.
    let html = r#"<a href="https://x.com"><b>https://x.com</b></a>"#;
    assert_eq!(run_tier2(html), "<https://x.com>\n");

    let report = PrescanReport::default();
    match tier1::run(html, &report, &tier1_friendly_options()) {
        Ok(markdown) => assert_eq!(
            markdown,
            run_tier2(html),
            "tier1 returned Ok with output that disagrees with tier2"
        ),
        Err(reason) => assert!(
            matches!(reason, BailReason::LinkAutolinkNestedMarkup),
            "expected LinkAutolinkNestedMarkup, got {reason:?}"
        ),
    }
}
