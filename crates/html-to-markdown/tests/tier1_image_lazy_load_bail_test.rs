//! Regression coverage for a Tier-1/Tier-2 divergence introduced by Tier-2's `<img>`
//! lazy-load fallback (`src/converter/handlers/image.rs`'s `resolve_effective_src`,
//! landed in commit `55699777d5`).
//!
//! When an `<img>`'s `src` is empty/whitespace-only or a `data:` URI, Tier-2 now
//! resolves the real image URL from `data-src`, `data-lazy-src`, `data-original`,
//! `data-srcset`, or `srcset` (in that precedence order — see `resolve_effective_src`'s
//! doc comment, including `pick_best_srcset_candidate`'s width/density-descriptor
//! comparison for the `srcset` attributes). Tier-1's `TagKind::Image` open handling
//! (`src/converter/tier1/scanner.rs`) still reads only `src`, which would silently
//! diverge on this shape instead of reproducing it. Reproducing `srcset`'s descriptor
//! grammar and the exact fallback precedence byte-for-byte in a single-pass scanner
//! duplicates non-trivial logic for a shape that is not on any hot path, so Tier-1 bails
//! instead (`BailReason::ImageLazyLoadSrc`) and lets Tier-2's fallback render it.
//!
//! Note: this worktree is pinned to a baseline commit that predates `55699777d5`, so
//! `TierStrategy::Tier2` here does NOT yet have the lazy-load fallback -- these tests
//! therefore assert directly on `tier1::run`'s `Err`/`Ok` (the actual defect surface)
//! rather than comparing against a live Tier-2 call, which would compare against the
//! OLD, pre-fallback Tier-2 behavior and prove nothing about the real divergence.

#![cfg(feature = "testkit")]

use html_to_markdown_rs::prescan::PrescanReport;
use html_to_markdown_rs::tier1::{self, BailReason};
use html_to_markdown_rs::{ConversionOptions, HighlightStyle, TierStrategy};

fn tier1_friendly_options() -> ConversionOptions {
    ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        ..ConversionOptions::default()
    }
}

fn run_tier1(html: &str) -> Result<String, BailReason> {
    tier1::run(html, &PrescanReport::default(), &tier1_friendly_options())
}

#[test]
fn empty_src_with_data_src_fallback_bails() {
    let html = r#"<img data-src="https://cdn.example.com/real.jpg" alt="Real image" />"#;
    let err = run_tier1(html).expect_err("empty src with a data-src fallback must bail, not emit an empty URL");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}

#[test]
fn blank_src_with_data_lazy_src_fallback_bails() {
    let html = r#"<img src="" data-lazy-src="https://x/a.png" alt="A" />"#;
    let err = run_tier1(html).expect_err("blank src with a data-lazy-src fallback must bail, not emit an empty URL");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}

#[test]
fn data_uri_src_with_srcset_fallback_bails() {
    let html =
        r#"<img src="data:image/gif;base64,R0lGOD" srcset="https://x/a.png 800w, https://x/b.png 1600w" alt="B" />"#;
    let err = run_tier1(html).expect_err("a data: src with a srcset fallback must bail, not keep the data: URI");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}

#[test]
fn data_uri_src_with_data_original_fallback_bails() {
    let html = r#"<img src="data:image/gif;base64,R0lGOD" data-original="https://x/real.png" alt="C" />"#;
    let err = run_tier1(html).expect_err("a data: src with a data-original fallback must bail");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}

#[test]
fn empty_src_with_data_srcset_fallback_bails() {
    let html = r#"<img src="" data-srcset="https://x/a.png 1x, https://x/b.png 2x" alt="D" />"#;
    let err = run_tier1(html).expect_err("empty src with a data-srcset fallback must bail");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}

#[test]
fn plain_img_with_populated_src_and_no_fallback_attrs_still_renders() {
    let html = r#"<img src="x.png" alt="a">"#;
    let rendered = run_tier1(html).expect("a plain <img src> with no lazy-load attributes must not bail");
    assert_eq!(rendered, "![a](x.png)\n");
}

#[test]
fn populated_src_is_trusted_even_alongside_a_srcset_attribute() {
    // ~keep Mirrors `resolve_effective_src`'s documented precedence: a genuinely
    // ~keep populated, non-`data:` `src` is trusted as-is regardless of what other
    // ~keep attributes are present -- only an empty-or-`data:` `src` triggers the
    // ~keep fallback search. Must NOT bail.
    let html = r#"<img src="real.jpg" srcset="placeholder.png 1x" alt="e" />"#;
    let rendered = run_tier1(html).expect("a populated, non-data: src must not bail even with a srcset present");
    assert_eq!(rendered, "![e](real.jpg)\n");
}

#[test]
fn whitespace_only_src_with_data_src_fallback_bails() {
    let html = r#"<img src="   " data-src="https://x/real.png" alt="f" />"#;
    let err = run_tier1(html).expect_err("a whitespace-only src with a data-src fallback must bail");
    assert!(
        matches!(err, BailReason::ImageLazyLoadSrc),
        "unexpected bail reason: {err:?}"
    );
}
