//! Timing harness for a single fixture.
//!
//! Methodology:
//! - 20 warmup iterations (discarded)
//! - Calibrate: find N such that N iterations take ~50 ms
//! - Nine runs of N iterations; report the median and MAD
//!
//! This avoids noisy single-sample measurements while keeping total run time
//! bounded for large fixtures.

use std::hint::black_box;
#[cfg(feature = "visitor")]
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use html_to_markdown_rs::options::ConversionOptions;
#[cfg(feature = "visitor")]
use html_to_markdown_rs::visitor::{HtmlVisitor, VisitorHandle};

use crate::schema::SAMPLES_PER_RUN;
use crate::stats;

/// No-op visitor used to isolate visitor-dispatch overhead from conversion cost.
///
/// Every method falls through to the default `VisitResult::Continue` from the trait,
/// so the only thing being measured against the no-visitor baseline is the cost of
/// building each `NodeContext` and acquiring the visitor mutex at every callback site.
#[derive(Debug, Default)]
#[cfg(feature = "visitor")]
pub struct NoOpVisitor;

#[cfg(feature = "visitor")]
impl HtmlVisitor for NoOpVisitor {}

/// Construct a fresh no-op visitor handle suitable for `ConversionOptions::visitor`.
#[cfg(feature = "visitor")]
pub fn new_noop_visitor_handle() -> VisitorHandle {
    Arc::new(Mutex::new(NoOpVisitor))
}

/// Target duration for a single calibration run.
const TARGET_CALIBRATION: Duration = Duration::from_millis(CALIBRATION_TARGET_MS);
/// Target duration used to derive an automatic iteration count.
pub const CALIBRATION_TARGET_MS: u64 = 50;
/// Warmup iteration count.
pub const WARMUP_ITERATIONS: u32 = 20;
/// Maximum wall time spent deriving an automatic iteration count.
pub const CALIBRATION_TIMEOUT_MS: u64 = 2_000;
/// Minimum iters per run to avoid degenerate timing.
const MIN_ITERS: u32 = 1;

/// Robust measurement retained for one fixture.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    /// Independently timed per-call samples in milliseconds.
    pub samples_ms: Vec<f64>,
    /// Median of `samples_ms`.
    pub median_ms: f64,
    /// Median absolute deviation of `samples_ms`.
    pub mad_ms: f64,
    /// Best of the first three batches for schema-v1 comparison compatibility.
    pub legacy_ms_best: f64,
    /// Output Markdown byte length.
    pub output_bytes: usize,
}

/// Run `convert(html, opts)` and return the output length.
///
/// Returns `None` when the conversion panics (known core bug on some fixtures).
fn run_once(html: &str, opts: Option<ConversionOptions>) -> Option<usize> {
    let html_owned = html.to_owned();
    match std::panic::catch_unwind(move || html_to_markdown_rs::convert(&html_owned, opts)) {
        Ok(Ok(result)) => Some(result.content.as_deref().map_or(0, str::len)),
        Ok(Err(_)) => Some(0),
        Err(_) => None,
    }
}

/// Benchmark a single HTML string with the given options.
///
/// Returns an all-zero measurement when the fixture panics during conversion.
pub fn run_one(html: &str, opts: Option<ConversionOptions>, iteration_override: Option<u32>) -> Measurement {
    let mut output_bytes = 0usize;
    let mut panicked = false;
    for _ in 0..WARMUP_ITERATIONS {
        if let Some(n) = run_once(html, opts.clone()) {
            output_bytes = n;
            black_box(output_bytes);
        } else {
            panicked = true;
            break;
        }
    }
    if panicked {
        return Measurement {
            samples_ms: vec![0.0; SAMPLES_PER_RUN],
            median_ms: 0.0,
            mad_ms: 0.0,
            legacy_ms_best: 0.0,
            output_bytes: 0,
        };
    }

    let iters = iteration_override.unwrap_or_else(|| {
        let start = Instant::now();
        let mut n: u32 = 1;
        loop {
            let t0 = Instant::now();
            for _ in 0..n {
                black_box(run_once(html, opts.clone()));
            }
            let elapsed = t0.elapsed();
            if elapsed >= TARGET_CALIBRATION {
                break;
            }
            if elapsed.as_nanos() > 0 {
                let factor = (TARGET_CALIBRATION.as_nanos() as f64 / elapsed.as_nanos() as f64).ceil() as u32;
                n = n.saturating_mul(factor).max(MIN_ITERS);
            } else {
                n = n.saturating_mul(2);
            }
            if start.elapsed() > Duration::from_millis(CALIBRATION_TIMEOUT_MS) {
                break;
            }
        }
        n.max(MIN_ITERS)
    });

    let mut samples_ms = Vec::with_capacity(SAMPLES_PER_RUN);
    for _ in 0..SAMPLES_PER_RUN {
        let t0 = Instant::now();
        for _ in 0..iters {
            black_box(run_once(html, opts.clone()));
        }
        let ms = t0.elapsed().as_nanos() as f64 / f64::from(iters) / 1_000_000.0;
        samples_ms.push(ms);
    }

    let legacy_ms_best = samples_ms[..3].iter().copied().fold(f64::INFINITY, f64::min);
    Measurement {
        median_ms: stats::median(&samples_ms),
        mad_ms: stats::mad(&samples_ms),
        legacy_ms_best,
        samples_ms,
        output_bytes,
    }
}
