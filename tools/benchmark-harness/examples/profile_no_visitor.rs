//! Profile driver for baseline (no visitor) on a single fixture.

#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)]

use html_to_markdown_rs::convert;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "tools/benchmark-harness/fixtures/real-world/wikipedia/small_html.html".to_string());
    let iters: u32 = std::env::var("ITERS").ok().and_then(|s| s.parse().ok()).unwrap_or(500);

    let html = std::fs::read_to_string(&path).expect("read fixture");

    eprintln!("profiling {} for {} iterations (no visitor)", path, iters);
    for _ in 0..iters {
        let _ = std::hint::black_box(convert(&html, None));
    }
}
