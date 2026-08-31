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

/// ~keep A single measurement is two independent noisy wall-clock samples divided by
/// ~keep each other, which is a poor instrument on a loaded machine: an unlucky
/// ~keep scheduler blip under either sample can push the ratio over
/// ~keep `MAX_DOUBLING_RATIO` even though the implementation is linear. A genuine
/// ~keep quadratic regression, by contrast, is a property of the algorithm rather
/// ~keep than the machine's mood, so it reproduces on every independent
/// ~keep re-measurement. The test therefore re-measures from scratch up to
/// ~keep `MAX_MEASUREMENT_ATTEMPTS` times and only fails if every single attempt
/// ~keep looks quadratic -- a suspected failure that does not reproduce is treated
/// ~keep as noise, not as a regression. 3 independent attempts, each already the
/// ~keep minimum of `REPEATS_PER_SAMPLE` runs, makes coincidental correlated noise
/// ~keep on every attempt astronomically unlikely without inflating the worst-case
/// ~keep (genuinely-quadratic) runtime past what a CI failure should reasonably cost.
const MAX_MEASUREMENT_ATTEMPTS: usize = 3;

/// ~keep Repeats per data point within a single attempt; the minimum of these is
/// ~keep used, which filters out one-off scheduler noise before the retry loop
/// ~keep above even needs to matter.
const REPEATS_PER_SAMPLE: usize = 3;

/// ~keep Base size chosen so the linear baseline stays in the tens-of-milliseconds
/// ~keep range: fast enough to keep this test quick, large enough that
/// ~keep quadratic blowup (which was ~4x/doubling, compounding across two
/// ~keep doublings below) is unmistakable rather than lost in noise.
const BASE_SIZE: usize = 20_000;

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

/// The two doubling ratios from one full small/medium/large measurement, plus
/// the raw timings needed to explain a failure.
struct DoublingMeasurement {
    small_secs: f64,
    medium_secs: f64,
    large_secs: f64,
    first_doubling_ratio: f64,
    second_doubling_ratio: f64,
}

impl DoublingMeasurement {
    /// Takes one full, independent measurement: builds fresh inputs and times
    /// the fastest of `REPEATS_PER_SAMPLE` conversions at each size.
    fn measure() -> Self {
        let small = "<".repeat(BASE_SIZE);
        let medium = "<".repeat(BASE_SIZE * 2);
        let large = "<".repeat(BASE_SIZE * 4);

        // Guard against dividing by (near-)zero on an extremely fast run.
        let small_secs = fastest_run(&small, REPEATS_PER_SAMPLE).as_secs_f64().max(1e-6);
        let medium_secs = fastest_run(&medium, REPEATS_PER_SAMPLE).as_secs_f64().max(1e-6);
        let large_secs = fastest_run(&large, REPEATS_PER_SAMPLE).as_secs_f64().max(1e-6);

        Self {
            small_secs,
            medium_secs,
            large_secs,
            first_doubling_ratio: medium_secs / small_secs,
            second_doubling_ratio: large_secs / medium_secs,
        }
    }

    /// Whether both doublings stayed under the quadratic-suspicion threshold.
    fn is_linear(&self) -> bool {
        self.first_doubling_ratio < MAX_DOUBLING_RATIO && self.second_doubling_ratio < MAX_DOUBLING_RATIO
    }

    /// A human-readable explanation of why this measurement looked quadratic.
    fn failure_reason(&self) -> String {
        let first_target = BASE_SIZE * 2;
        let second_source = BASE_SIZE * 2;
        let second_target = BASE_SIZE * 4;
        let first_doubling_ratio = self.first_doubling_ratio;
        let second_doubling_ratio = self.second_doubling_ratio;
        let small_secs = self.small_secs;
        let medium_secs = self.medium_secs;
        let large_secs = self.large_secs;
        format!(
            "doubling {BASE_SIZE} -> {first_target} took {first_doubling_ratio:.1}x as long \
             ({small_secs:.4}s -> {medium_secs:.4}s); doubling {second_source} -> {second_target} \
             took {second_doubling_ratio:.1}x as long ({medium_secs:.4}s -> {large_secs:.4}s); \
             expected roughly linear scaling (<{MAX_DOUBLING_RATIO}x each step)"
        )
    }
}

#[test]
fn bare_lt_run_scales_linearly_not_quadratically() {
    let mut failures = Vec::with_capacity(MAX_MEASUREMENT_ATTEMPTS);

    for attempt in 1..=MAX_MEASUREMENT_ATTEMPTS {
        let measurement = DoublingMeasurement::measure();
        if measurement.is_linear() {
            return;
        }
        failures.push(format!("attempt {attempt}: {}", measurement.failure_reason()));
    }

    panic!(
        "bare-'<' conversion scaled super-linearly on all {MAX_MEASUREMENT_ATTEMPTS} independent \
         measurement attempts -- transient scheduler noise does not reproduce this consistently, so \
         this points at a genuine return of the quadratic `strip_hidden_elements` bug:\n{}",
        failures.join("\n")
    );
}
