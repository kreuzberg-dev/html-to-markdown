//! Regression coverage for the leading-whitespace-on-continuation-lines fixpoint defect
//! (`CommonMark` spec example 182: `<![CDATA[ ... ]]>` whose body is indented source code).
//!
//! `normalize_whitespace_cow` used to collapse a run of spaces/tabs immediately after an
//! embedded `\n` down to a single space. That single space was not a fixed point: a
//! `CommonMark`-compliant parser drops a paragraph continuation line's leading whitespace
//! entirely on reparse (spec 4.9), so re-converting the rendered Markdown lost the space and
//! shrank the run again -- forever, on every subsequent pass. `text_node.rs` now runs
//! `normalize_block_whitespace_cow` on the already-boundary-trimmed core, collapsing such a
//! run straight to nothing so the very first pass already matches what the round trip forces.

#![allow(missing_docs)]
#![cfg(feature = "testkit")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn convert_with(html: &str, options: &ConversionOptions) -> String {
    convert(html, Some(options.clone()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn escaping_options() -> ConversionOptions {
    ConversionOptions {
        escape_misc: true,
        escape_asterisks: true,
        escape_underscores: true,
        ..Default::default()
    }
}

fn render_markdown_to_html(md: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.render.r#unsafe = true;
    comrak::markdown_to_html(md, &options)
}

/// Convert twice through the round trip (`html -> md1 -> html2 -> md2`) and assert
/// `md1 == md2` -- the direct idempotency check.
fn assert_round_trip_is_a_fixpoint(html: &str, options: &ConversionOptions, context: &str) {
    let md1 = convert_with(html, options);
    let html2 = render_markdown_to_html(&md1);
    let md2 = convert_with(&html2, options);
    assert_eq!(
        md1, md2,
        "{context}: round trip is not a fixpoint\nmd1: {md1:?}\nmd2: {md2:?}"
    );
}

#[test]
fn commonmark_spec_example_182_reaches_a_conversion_fixpoint() {
    // ~keep The exact CommonMark spec example 182 input: a top-level CDATA section whose
    // ~keep body is indented source code, followed by a sibling paragraph.
    let html = concat!(
        "<![CDATA[\n",
        "function matchwo(a,b)\n",
        "{\n",
        "  if (a < b && a < 0) then {\n",
        "    return 1;\n",
        "\n",
        "  } else {\n",
        "\n",
        "    return 0;\n",
        "  }\n",
        "}\n",
        "]]>\n",
        "<p>okay</p>\n",
    );
    assert_round_trip_is_a_fixpoint(html, &escaping_options(), "spec example 182");
}

#[test]
fn indented_multiline_div_text_with_blank_lines_is_idempotent() {
    // ~keep A non-CDATA shape exercising the same code path: a single text node with
    // ~keep multiple indented continuation lines and embedded blank lines, directly under
    // ~keep a plain block element rather than parsed as a bogus-comment-adjacent CDATA body.
    let html = "<div>line one\n    line two\n\n    line three\n  line four</div>";
    assert_round_trip_is_a_fixpoint(html, &escaping_options(), "indented multiline div text");
}

#[test]
fn converting_twice_produces_identical_output_for_indented_continuation_lines() {
    // ~keep Literal "convert twice" framing: the second conversion is fed the first
    // ~keep conversion's own Markdown rendered back to HTML, and must reproduce it exactly.
    let html = "<p>foo\n    bar\n\n    baz</p>";
    let options = escaping_options();
    let first = convert_with(html, &options);
    let rendered = render_markdown_to_html(&first);
    let second = convert_with(&rendered, &options);
    assert_eq!(first, second, "first: {first:?}\nsecond: {second:?}");
}

#[test]
fn tier1_bails_on_cdata_and_falls_back_to_the_fixed_tier2_output() {
    // ~keep Tier-1 does not support CDATA sections at all (`BailReason::Cdata`) and always
    // ~keep falls back to Tier-2 for this shape, so forcing `TierStrategy::Tier1` here must
    // ~keep reproduce Tier-2's (fixed) output exactly rather than diverging.
    let html = concat!(
        "<![CDATA[\n",
        "function matchwo(a,b)\n",
        "{\n",
        "  if (a < b && a < 0) then {\n",
        "    return 1;\n",
        "\n",
        "  } else {\n",
        "\n",
        "    return 0;\n",
        "  }\n",
        "}\n",
        "]]>\n",
        "<p>okay</p>\n",
    );
    let mut options = escaping_options();
    options.tier_strategy = TierStrategy::Tier1;
    let tier1_forced = convert_with(html, &options);
    options.tier_strategy = TierStrategy::Tier2;
    let tier2 = convert_with(html, &options);
    assert_eq!(
        tier1_forced, tier2,
        "tier1 should fall back to tier2's output for CDATA"
    );
    assert!(
        tier1_forced.contains("return 1;\n\n"),
        "expected the blank line inside the indented block to survive: {tier1_forced:?}"
    );
    assert!(
        !tier1_forced.contains("\n "),
        "no continuation line should retain leading indentation: {tier1_forced:?}"
    );
}
