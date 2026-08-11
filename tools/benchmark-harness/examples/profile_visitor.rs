//! Profile driver for visitor overhead on a single fixture.
//!
//! Used with `samply record -- ./target/release/examples/profile_visitor <fixture>`.
//! Defaults to a large real-world wikipedia page that exercises the visitor heavily.

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use std::sync::{Arc, Mutex};

use html_to_markdown_bench::bench::NoOpVisitor;
use html_to_markdown_rs::convert;
use html_to_markdown_rs::options::ConversionOptions;
use html_to_markdown_rs::visitor::VisitorHandle;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "tools/benchmark-harness/fixtures/real-world/wikipedia/small_html.html".to_string());
    let iters: u32 = std::env::var("ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(500);

    let html = std::fs::read_to_string(&path).expect("read fixture");
    let visitor: VisitorHandle = Arc::new(Mutex::new(NoOpVisitor));
    let opts = ConversionOptions {
        visitor: Some(visitor),
        ..ConversionOptions::default()
    };

    eprintln!("profiling {} for {} iterations", path, iters);
    for _ in 0..iters {
        let _ = std::hint::black_box(convert(&html, Some(opts.clone())));
    }
}
