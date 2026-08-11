//! Profile a single Tier-2 fixture repeatedly to analyze allocations.
//!
//! Run with:
//!   cargo run --release --example profile_tier2_fixture -p html-to-markdown-bench
//!
//! Then analyze with:
//!   cargo flamegraph --release --example profile_tier2_fixture -p html-to-markdown-bench
//!

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let html = std::fs::read_to_string("tools/benchmark-harness/fixtures/real-world/wikipedia/medium_python.html")?;

    let opts = html_to_markdown_rs::ConversionOptions {
        tier_strategy: html_to_markdown_rs::TierStrategy::Auto,
        extract_metadata: false,
        ..html_to_markdown_rs::ConversionOptions::default()
    };

    let _ = html_to_markdown_rs::convert(&html, Some(opts.clone()))?;

    const ITERS: u32 = 100;
    let start = Instant::now();

    for _ in 0..ITERS {
        let _ = html_to_markdown_rs::convert(&html, Some(opts.clone()))?;
    }

    let elapsed = start.elapsed();
    let bytes_total = (html.len() as u64) * u64::from(ITERS);
    let mb_per_sec = (bytes_total as f64) / (1024.0 * 1024.0) / elapsed.as_secs_f64();

    println!(
        "Converted {} iterations in {:.3}s ({:.1} MB/s)",
        ITERS,
        elapsed.as_secs_f64(),
        mb_per_sec
    );

    Ok(())
}
