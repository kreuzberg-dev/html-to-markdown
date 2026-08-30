// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! A `<br>` inside a code context must not emit a `newline_style` marker.
//!
//! A code span reproduces its content literally, so the marker is not syntax there -- it is
//! content. Under `newline_style="backslash"` that put a literal `\` inside the span, and
//! inside a fenced block it put one into the code itself; under the two-space style it
//! injected trailing spaces into the code. `CommonMark` treats a line ending inside a code
//! span as a space and gives it no hard-break meaning at all
//! (<https://spec.commonmark.org/spec#code-spans>).
//!
//! This is the same rule the table-cell branch of `line_break.rs` already applies for the
//! same reason: a cell cannot carry a hard break either, so `newline_style` is never
//! consulted there.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

fn convert(html: &str, style: NewlineStyle) -> String {
    html_to_markdown_rs::convert(
        html,
        Some(ConversionOptions {
            newline_style: style,
            ..Default::default()
        }),
    )
    .expect("conversion should succeed")
    .content
    .unwrap_or_default()
}

#[test]
fn should_not_put_a_backslash_marker_inside_an_inline_code_span() {
    for html in [
        "<p><code>A<br>B</code></p>",
        "<code>A<br>B</code>",
        "<p><code>A<br>B</code>C</p>",
        "<p><kbd>A<br>B</kbd></p>",
        "<p><samp>A<br>B</samp></p>",
    ] {
        let out = convert(html, NewlineStyle::Backslash);
        assert!(
            !out.contains('\\'),
            "marker leaked into code content for {html}: {out:?}"
        );
    }
}

#[test]
fn should_not_put_a_marker_inside_a_fenced_code_block() {
    // ~keep The worst of the set: the fence reproduces its content verbatim, so a marker
    // ~keep here is not invisible under either style -- it is a character in the user's code.
    assert_eq!(
        convert("<pre><code>A<br>B</code></pre>", NewlineStyle::Backslash),
        "```\nA\nB\n```\n"
    );
    assert_eq!(
        convert("<pre><code>A<br>B</code></pre>", NewlineStyle::Spaces),
        "```\nA\nB\n```\n"
    );
}

#[test]
fn should_emit_the_same_code_content_under_both_newline_styles() {
    // ~keep `newline_style` selects hard-break *syntax*. A code span has no hard breaks, so
    // ~keep the option must not reach it -- the two styles have to agree byte for byte.
    for html in [
        "<p><code>A<br>B</code></p>",
        "<pre><code>A<br>B</code></pre>",
        "<p><code>A<br><br>B</code></p>",
        "<blockquote><p><code>A<br>B</code></p></blockquote>",
    ] {
        assert_eq!(
            convert(html, NewlineStyle::Backslash),
            convert(html, NewlineStyle::Spaces),
            "styles disagree for {html}"
        );
    }
}

#[test]
fn should_still_emit_a_real_break_between_two_code_spans() {
    // ~keep The counterweight: the <br> is BETWEEN spans, not inside one, so it is an
    // ~keep ordinary hard break and must keep its marker.
    let out = convert("<p><code>a</code><br><code>b</code></p>", NewlineStyle::Backslash);
    assert_eq!(out, "`a`\\\n`b`\n");
}
