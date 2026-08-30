//! Regression test for the quadratic-time bug in `strip_hidden_elements`
//! (`crates/html-to-markdown/src/converter/utility/preprocessing.rs`).
//!
//! A run of unterminated `<` characters (a `<` with no matching `>`) made that
//! preprocessing pass re-scan from scratch, all the way to end-of-input, for
//! every single `<` it encountered, turning the whole conversion O(n^2) in the
//! number of such characters. This is a denial-of-service vector because the
//! library converts arbitrary, untrusted HTML.

use std::time::Instant;

use html_to_markdown_rs::convert;

/// ~keep Threshold rationale: a truly linear implementation doubles the input size
/// ~keep and sees roughly a 2x wall-clock increase; the (already fixed) quadratic
/// ~keep implementation saw ~4x per doubling. A ratio ceiling of 3.0 comfortably
/// ~keep separates "linear, plus noise from a loaded CI box" from "quadratic",
/// ~keep without being tight enough to flake on timing jitter.
const MAX_DOUBLING_RATIO: f64 = 3.0;

/// Times a single `convert` call on `html`, returning the elapsed duration.
fn time_convert(html: &str) -> std::time::Duration {
    let start = Instant::now();
    let result = convert(html, None);
    let elapsed = start.elapsed();
    assert!(result.is_ok(), "conversion should not fail on bare '<' input");
    elapsed
}

/// Runs `time_convert` `repeats` times and returns the minimum duration, to
/// reduce flakiness from scheduler noise on a loaded machine.
fn fastest_run(html: &str, repeats: usize) -> std::time::Duration {
    (0..repeats).map(|_| time_convert(html)).min().unwrap()
}

#[test]
fn bare_lt_run_scales_linearly_not_quadratically() {
    // ~keep Base size chosen so the linear baseline stays in the tens-of-milliseconds
    // ~keep range: fast enough to keep this test quick, large enough that
    // ~keep quadratic blowup (which was ~4x/doubling, compounding across two
    // ~keep doublings below) is unmistakable rather than lost in noise.
    const BASE_SIZE: usize = 20_000;

    let small = "<".repeat(BASE_SIZE);
    let medium = "<".repeat(BASE_SIZE * 2);
    let large = "<".repeat(BASE_SIZE * 4);

    // Guard against dividing by (near-)zero on an extremely fast run.
    let small_secs = fastest_run(&small, 3).as_secs_f64().max(1e-6);
    let medium_secs = fastest_run(&medium, 3).as_secs_f64().max(1e-6);
    let large_secs = fastest_run(&large, 3).as_secs_f64().max(1e-6);

    let first_doubling_ratio = medium_secs / small_secs;
    let second_doubling_ratio = large_secs / medium_secs;

    assert!(
        first_doubling_ratio < MAX_DOUBLING_RATIO,
        "doubling the bare-'<' run from {BASE_SIZE} to {} took {first_doubling_ratio:.1}x as long \
         ({small_secs:.4}s -> {medium_secs:.4}s); expected roughly linear scaling \
         (<{MAX_DOUBLING_RATIO}x), which suggests the quadratic bug has returned",
        BASE_SIZE * 2,
    );
    assert!(
        second_doubling_ratio < MAX_DOUBLING_RATIO,
        "doubling the bare-'<' run from {} to {} took {second_doubling_ratio:.1}x as long \
         ({medium_secs:.4}s -> {large_secs:.4}s); expected roughly linear scaling \
         (<{MAX_DOUBLING_RATIO}x), which suggests the quadratic bug has returned",
        BASE_SIZE * 2,
        BASE_SIZE * 4,
    );
}
