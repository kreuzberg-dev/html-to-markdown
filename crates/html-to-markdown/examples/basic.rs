// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: examples print by design

//! Example: Basic HTML to Markdown conversion

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn main() {
    let html = "<h1>Hello World</h1><p>This is a <strong>test</strong>.</p>";

    match convert(html, None) {
        Ok(markdown) => {
            println!("HTML:");
            println!("{html}");
            println!("\nMarkdown:");
            println!("{markdown}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
