//! Regression coverage for the `TagKind::Block` tag names that Tier-2's `main.rs`
//! dispatch match does not route through a dedicated separator-emitting handler by
//! default: `<address>`, `<search>`, `<hgroup>`, `<center>`, `<colgroup>`, `<col>`,
//! `<base>`, `<html>`, `<body>`.
//!
//! `<address>`, `<search>`, `<hgroup>`, and `<center>` are content-bearing block
//! containers -- two adjacent elements used to merge into a single run of text with no
//! separator at all (`"foobar\n"` instead of `"foo\n\nbar\n"`), which is structural
//! content loss. They are now routed to `block::div::handle` in `main.rs`, the same
//! handler `<div>` uses, so they get the same leading/trailing `"\n\n"` block
//! separator `<div>` does.
//!
//! `<colgroup>`/`<col>` (table-internal metadata), `<base>` (void metadata), and
//! `<html>`/`<body>` (document wrappers whose children ARE the document) are
//! deliberately left as passthrough: they still fall through to Tier-2's `_ =>` arm
//! (`block::unknown::handle`) or, for `<html>`/`<body>`,
//! `block::container::handle_structural_container` -- both of which walk children with
//! NO separator of their own. Tier-1's `block_container_is_passthrough` in
//! `scanner.rs` mirrors exactly this split.

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
fn adjacent_address_elements_get_a_separator() {
    let html = "<address>foo</address><address>bar</address>";
    assert_eq!(
        run_tier2(html),
        "foo\n\nbar\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(run_tier1(html), "foo\n\nbar\n");
    assert_tier1_matches_tier2(html);
}

#[test]
fn address_between_real_block_siblings_still_gets_their_separators() {
    // ~keep The separator here already came from the SURROUNDING `<p>`/`<div>` handlers,
    // ~keep not from `<address>` itself -- unaffected by routing `<address>` to
    // ~keep `div::handle`, since a single `<address>` sibling never needed its own
    // ~keep leading/trailing separator when one was already present.
    assert_tier1_matches_tier2("<p>a</p><address>b</address><p>c</p>");
    assert_tier1_matches_tier2("<div>a</div><address>b</address><div>c</div>");
}

#[test]
fn lone_address_element_renders_plain() {
    assert_eq!(run_tier2("<address>only</address>"), "only\n");
    assert_tier1_matches_tier2("<address>only</address>");
}

#[test]
fn address_inside_table_cell_gets_a_separator_but_tier1_diverges() {
    // ~keep Routing `<address>` to `block::div::handle` means it now inherits
    // ~keep `div::handle`'s `is_table_continuation` behaviour: Tier-2 emits a single
    // ~keep space between the two cell contents via `emit_table_cell_break`. This is a
    // ~keep genuine, pre-existing Tier-1/Tier-2 divergence -- NOT introduced by this
    // ~keep fix and NOT specific to `<address>`: plain `<div>` has the identical gap
    // ~keep (verified separately: `<table><tr><td><div>a</div><div>b</div></td></tr></table>`
    // ~keep also renders `"| a b |\n| --- |\n"` under Tier-2 but
    // ~keep `"| a   b |\n| ----- |\n"` under Tier-1). Tier-1's generic `TagKind::Block`
    // ~keep in-cell handling (`scanner.rs`) pushes `"  \n"` before the next block
    // ~keep container's content, which the table-cell finalizer turns into three spaces
    // ~keep instead of Tier-2's one. `<address>` simply inherits this existing `<div>`
    // ~keep gap by inheriting its handler; fixing the underlying `<div>` divergence is
    // ~keep out of scope here.
    let html = "<table><tr><td><address>a</address><address>b</address></td></tr></table>";
    assert_eq!(
        run_tier2(html),
        "| a b |\n| --- |\n",
        "tier2 ground truth changed; update this test"
    );
    assert_eq!(
        run_tier1(html),
        "| a   b |\n| ----- |\n",
        "tier1 ground truth changed; update this test"
    );
}

#[test]
fn search_hgroup_center_now_get_a_separator() {
    for tag in ["search", "hgroup", "center"] {
        let html = format!("<{tag}>foo</{tag}><{tag}>bar</{tag}>");
        assert_eq!(
            run_tier2(&html),
            "foo\n\nbar\n",
            "tier2 ground truth changed for <{tag}>; update this test"
        );
        assert_tier1_matches_tier2(&html);
    }
}

#[test]
fn colgroup_still_has_no_separator() {
    // ~keep `<colgroup>` is table-internal metadata (it never carries visible content of
    // ~keep its own); left as passthrough deliberately -- see this file's module doc.
    let html = "<colgroup>foo</colgroup><colgroup>bar</colgroup>";
    assert_eq!(
        run_tier2(html),
        "foobar\n",
        "tier2 ground truth changed; update this test"
    );
    assert_tier1_matches_tier2(html);
}

#[test]
fn col_base_html_body_still_have_no_separator() {
    // ~keep `<col>` is table-internal metadata and void; `<base>` is void document
    // ~keep metadata; `<html>`/`<body>` are document wrappers whose children ARE the
    // ~keep document. All four are left as passthrough deliberately -- see this file's
    // ~keep module doc. `<col>`/`<base>` are void elements, so they are exercised
    // ~keep without closing tags (a closing tag on a void element desyncs Tier-1's
    // ~keep nesting depth tracking and is not a realistic input shape).
    let cases = [
        "<col>foo<col>bar",
        "<base>foo<base>bar",
        "<html><body>foo</body></html><html><body>bar</body></html>",
        "<body>foo</body><body>bar</body>",
    ];
    for html in cases {
        assert_eq!(
            run_tier2(html),
            "foobar\n",
            "tier2 ground truth changed for {html:?}; update this test"
        );
        assert_tier1_matches_tier2(html);
    }
}

#[test]
fn div_still_gets_its_own_separator_unaffected_by_the_fix() {
    assert_tier1_matches_tier2("<div>foo</div><div>bar</div>");
}
