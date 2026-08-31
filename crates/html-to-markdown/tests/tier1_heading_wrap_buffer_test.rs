//! Regression tests for two Tier-1 defects found on real rustdoc HTML:
//!
//! 1. A heading (`<h2>`-`<h6>`) inside `<summary>`/`<figcaption>` spliced its `#` prefix
//!    into the middle of unrelated, already-emitted text. Root cause: `open_heading` and
//!    `close_heading` (`src/converter/tier1/scanner.rs`) hardcoded `state.output` for
//!    their leading-separator and prefix-insertion steps, instead of routing through
//!    `Tier1State::cell_or_output_mut`. A heading's `content_start` is captured from
//!    whichever buffer is active AT OPEN TIME (the summary/figcaption accumulation
//!    buffer, when one is active) — but the old code then read and mutated `state.output`
//!    at CLOSE time, treating that small buffer-relative offset as an offset into the
//!    much larger real document output.
//! 2. A text node's trailing bare `\n` (no accompanying space, not a `\n\n` run)
//!    immediately before an inline sibling tag dropped the separating space entirely
//!    instead of collapsing it to one, when Tier-2's `text_node.rs` "trailing single
//!    newline" follow-up step (mirrored here by `trailing_single_newline_join`) would
//!    have put a space back. Only surfaced once (1) stopped masking it with corruption.

#![cfg(feature = "testkit")]

mod support;

use html_to_markdown_rs::options::HighlightStyle;
use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn tier1(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier1,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

fn tier2(html: &str) -> String {
    let opts = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        extract_metadata: false,
        ..ConversionOptions::default()
    };
    convert(html, Some(opts)).unwrap().content.unwrap_or_default()
}

/// Assert Tier-1 and Tier-2 produce byte-identical output for `html`.
fn assert_matches_tier2(html: &str) {
    let t1 = tier1(html);
    let t2 = tier2(html);
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2\ninput: {html:?}\ntier1: {t1:?}\ntier2: {t2:?}"
    );
}

/// Minimized trigger for defect (1): a heading inside `<summary>` with preceding
/// sibling text in the same accumulation buffer, so its `content_start` is non-zero.
#[test]
fn heading_inside_summary_does_not_corrupt_preceding_text() {
    assert_matches_tier2("<details><summary>impl <h3>Sample</h3></summary></details>");
}

/// Same shape, `<figcaption>` instead of `<summary>` -- both share the wrap-buffer stack.
#[test]
fn heading_inside_figcaption_does_not_corrupt_preceding_text() {
    assert_matches_tier2("<figure><figcaption>impl <h4>Sample</h4></figcaption></figure>");
}

/// A heading inside `<summary>` that is itself the FIRST content (`content_start` == 0
/// in the buffer) must still get its separator/prefix scoped to the buffer, matching
/// Tier-2's `push_heading` leading-separator step against the local `content` buffer.
#[test]
fn heading_first_in_summary_buffer() {
    assert_matches_tier2("<details><summary><h3>Sample</h3></summary></details>");
}

/// Two headings inside the same `<summary>`, forcing a second, non-zero `content_start`.
#[test]
fn two_headings_inside_summary() {
    assert_matches_tier2("<details><summary><h3>One</h3> and <h4>Two</h4></summary></details>");
}

/// A heading inside a table cell keeps its own dedicated (no-prefix, no-separator)
/// behaviour even though `cell_or_output_mut` also routes summary buffers -- table
/// cells and summary buffers must not be conflated.
#[test]
fn heading_inside_table_cell_unaffected() {
    assert_matches_tier2("<table><tr><td>x<h3>Sample</h3>y</td></tr></table>");
}

/// A heading inside a summary that is ITSELF inside a table cell: `in_table_cell()`
/// must still win the "no prefix" branch, writing into the summary's buffer (summary
/// takes buffer-selection priority) with no `#`.
#[test]
fn heading_inside_summary_inside_table_cell() {
    assert_matches_tier2("<table><tr><td><details><summary><h3>Sample</h3></summary></details></td></tr></table>");
}

/// Defect (2): a bare trailing `\n` before an inline sibling, inside a `<p>` at the very
/// start of the paragraph's own content -- Tier-2's `push_heading`-style leading-sep
/// scoping doesn't apply here, but its `has_trailing_single_newline` follow-up does.
#[test]
fn trailing_bare_newline_before_inline_tag_in_paragraph() {
    assert_matches_tier2("<p>The number of\n<kbd>#</kbd> you use</p>");
}

/// Same shape as above but the paragraph is the SECOND paragraph in its container, so
/// the whole document buffer (not just this paragraph's own content) already ends in a
/// blank line when the text node is processed. This is the case that a naive
/// `state.output.ends_with("\n\n")` check gets wrong: it must be scoped to the nearest
/// `<p>`/`<div>` frame's own `content_start`, mirroring Tier-2's `ctx.block_content_start`.
#[test]
fn trailing_bare_newline_second_paragraph_after_blank_line() {
    assert_matches_tier2(
        "<div class=\"docblock\"><p>Calls <code>x</code>.</p>\n<p>That is, of\n<code>y</code> chooses.</p></div>",
    );
}

/// `<span>` is a hardcoded no-join exception in Tier-2's source.
#[test]
fn trailing_bare_newline_before_span_has_no_join() {
    assert_matches_tier2("<p>foo\n<span>bar</span></p>");
}

/// A bare trailing newline with no following sibling at all (text is the last child).
#[test]
fn trailing_bare_newline_no_next_sibling() {
    assert_matches_tier2("<div>foo\n</div>");
}

/// A bare trailing newline before an `<em>`/`<strong>` wrapper (Tier-2's
/// `inline_depth`-incrementing tags) outside any `<p>` ancestor.
#[test]
fn trailing_bare_newline_before_emphasis_outside_paragraph() {
    assert_matches_tier2("<div>foo\n<em>bar</em></div>");
}

/// Real-world regression: the exact rustdoc fixture that originally surfaced defect (1)
/// -- `## [Samp#### #### #### le](#) ...` -- under the options combination that routes
/// `Auto` into Tier-1 (`extract_metadata: false`, `highlight_style: None`).
#[test]
fn rustdoc_struct_sample_fixture_matches_tier2() {
    let path = support::corpus_root().join("docgen-rustdoc/struct-sample.html");
    let html = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let opts = ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    let t1 = convert(&html, Some(opts.clone())).unwrap().content.unwrap_or_default();
    let t2 = convert(
        &html,
        Some(ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..opts
        }),
    )
    .unwrap()
    .content
    .unwrap_or_default();
    assert_eq!(t1, t2, "tier1 diverged from tier2 on docgen-rustdoc/struct-sample.html");
    assert!(
        !t1.contains("Samp####"),
        "corruption regression: heading `#` splice reappeared in struct-sample.html output"
    );
}

/// Same real-world regression, second fixture (`enum-samplestate.html`), confirming the
/// fix is not overfit to the first document's exact structure.
#[test]
fn rustdoc_enum_samplestate_fixture_matches_tier2() {
    let path = support::corpus_root().join("docgen-rustdoc/enum-samplestate.html");
    let html = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
    let opts = ConversionOptions {
        extract_metadata: false,
        highlight_style: HighlightStyle::None,
        tier_strategy: TierStrategy::Tier1,
        ..ConversionOptions::default()
    };
    let t1 = convert(&html, Some(opts.clone())).unwrap().content.unwrap_or_default();
    let t2 = convert(
        &html,
        Some(ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            ..opts
        }),
    )
    .unwrap()
    .content
    .unwrap_or_default();
    assert_eq!(
        t1, t2,
        "tier1 diverged from tier2 on docgen-rustdoc/enum-samplestate.html"
    );
}
