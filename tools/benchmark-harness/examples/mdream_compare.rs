//! One-shot comparison: mdream vs html-to-markdown on the bench fixtures.
//!
//! Run with:
//!   cargo run --release --example mdream_compare -p html-to-markdown-bench
//!
//! Reports:
//!   - per-fixture throughput (mdream MB/s, h2m MB/s, ratio)
//!   - output size delta (mdream chars vs h2m chars)
//!   - per-fixture diff sample (first divergent line)
//!
//! Purpose: data for the "drop our scanner, use mdream" decision. We want
//! mdream's scanner speed but only if its output is semantically close enough
//! to ours on most real-world docs.

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use std::path::PathBuf;
use std::time::Instant;

use html_to_markdown_bench::fixture::Loader;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};
use mdream::{HTMLToMarkdownOptions, html_to_markdown as mdream_convert};

const ITERS: u32 = 5;

fn main() -> anyhow::Result<()> {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let loader = Loader::new(fixtures_dir);
    let fixtures = loader.load(None)?;

    println!(
        "{:<48} {:>10} {:>10} {:>10} {:>9} {:>10} {:>10} {:>8}",
        "fixture", "bytes", "h2m MB/s", "md MB/s", "md×", "h2m chars", "md chars", "size%"
    );
    println!("{:-<118}", "");

    let h2m_opts = ConversionOptions {
        tier_strategy: TierStrategy::Auto,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    let md_opts = HTMLToMarkdownOptions::default();

    let mut totals = Vec::new();

    for fix in &fixtures {
        let html = std::fs::read_to_string(&fix.path)?;
        let bytes = html.len() as f64;

        let _ = convert(&html, Some(h2m_opts.clone()))?;
        let _ = mdream_convert(&html, md_opts.clone());

        let mut h2m_best = f64::INFINITY;
        let mut md_best = f64::INFINITY;
        let mut h2m_out = String::new();
        let mut md_out = String::new();

        for _ in 0..ITERS {
            let t0 = Instant::now();
            let r = convert(&html, Some(h2m_opts.clone()))?;
            let dt = t0.elapsed().as_secs_f64() * 1000.0;
            h2m_best = h2m_best.min(dt);
            h2m_out = r.content.unwrap_or_default();

            let t0 = Instant::now();
            md_out = mdream_convert(&html, md_opts.clone());
            let dt = t0.elapsed().as_secs_f64() * 1000.0;
            md_best = md_best.min(dt);
        }

        let h2m_mbps = bytes / 1.0e6 / (h2m_best / 1000.0);
        let md_mbps = bytes / 1.0e6 / (md_best / 1000.0);
        let ratio = md_mbps / h2m_mbps;
        let size_ratio = md_out.len() as f64 / h2m_out.len().max(1) as f64;

        println!(
            "{:<48} {:>10} {:>10.1} {:>10.1} {:>8.2}× {:>10} {:>10} {:>7.0}%",
            fix.rel_path,
            fix.bytes,
            h2m_mbps,
            md_mbps,
            ratio,
            h2m_out.len(),
            md_out.len(),
            size_ratio * 100.0
        );

        totals.push((fix.rel_path.clone(), h2m_mbps, md_mbps, ratio, size_ratio));
    }

    println!();
    println!("=== Aggregate ===");
    let n = totals.len() as f64;
    let h2m_avg: f64 = totals.iter().map(|t| t.1).sum::<f64>() / n;
    let md_avg: f64 = totals.iter().map(|t| t.2).sum::<f64>() / n;
    let ratio_geomean: f64 = totals.iter().map(|t| t.3.ln()).sum::<f64>() / n;
    println!("h2m avg MB/s:    {:.1}", h2m_avg);
    println!("mdream avg MB/s: {:.1}", md_avg);
    println!("mdream speedup geomean: {:.2}×", ratio_geomean.exp());

    println!();
    println!("=== Size variance (mdream / h2m) ===");
    let mut sizes: Vec<_> = totals.iter().map(|t| (t.0.clone(), t.4)).collect();
    sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Smallest mdream/h2m size ratios (mdream drops content):");
    for (name, r) in sizes.iter().take(5) {
        println!("  {:.0}%  {}", r * 100.0, name);
    }
    println!("Largest mdream/h2m size ratios (mdream adds content):");
    for (name, r) in sizes.iter().rev().take(5) {
        println!("  {:.0}%  {}", r * 100.0, name);
    }

    Ok(())
}
