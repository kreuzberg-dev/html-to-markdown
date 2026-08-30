// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![allow(clippy::print_stdout, clippy::print_stderr, clippy::dbg_macro)] // ~keep: tests print by design

//! Every `CommonMark` spec example, as a conversion fixpoint.
//!
//! `commonmark_compliance_test.rs` compares our Markdown against the spec's Markdown, which
//! is only a fair oracle where the spec's rendering is the *only* valid one -- so it skips 23
//! of 24 sections and actually exercises 131 of 652 examples. That is the right call for that
//! comparison (ATX vs setext, `-` vs `*` bullets and so on are all conformant), but it leaves
//! 521 spec inputs untouched.
//!
//! The reason a string comparison cannot cover them is that `CommonMark` frequently admits
//! several equally valid renderings of the same document -- ATX versus setext headings, `-`
//! versus `*` bullets, indented versus fenced code -- and this crate's defaults pick one. Not
//! matching the spec's choice is conformant, so those skips are correct, not lazy.
//!
//! This file uses those same 652 examples against an oracle that needs no canonical form:
//! take the spec's HTML, convert it, render that Markdown back to HTML with comrak, and
//! convert again. That asks whether OUR choice is stable, never whether it equals the spec's,
//! so it applies to every section. A converter that loses or corrupts structure is not a
//! fixpoint under it.
//!
//! The spec's HTML side is the input, so this covers constructs the string comparison cannot:
//! lists, links, images, emphasis, block quotes, headings, code spans, autolinks, raw HTML.

use std::collections::BTreeMap;

use html_to_markdown_rs::{ConversionOptions, NewlineStyle, convert};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpecExample {
    html: String,
    example: u32,
    section: String,
}

fn spec_examples() -> Vec<SpecExample> {
    let raw = include_str!("../../../packages/python/tests/commonmark_spec.json");
    serde_json::from_str(raw).expect("commonmark_spec.json should parse")
}

fn to_markdown(html: &str, options: &ConversionOptions) -> Option<String> {
    convert(html, Some(options.clone())).ok()?.content
}

fn render(md: &str) -> String {
    let mut options = comrak::Options::default();
    // ~keep The crate emits GFM tables and strikethrough, so a renderer that cannot parse them
    // ~keep would report a divergence that is its own blind spot rather than our bug.
    options.extension.table = true;
    options.extension.strikethrough = true;
    // ~keep GFM autolinking is deliberately NOT enabled: core CommonMark's Autolinks section
    // ~keep exists to show a bare URL in TEXT is not a link without `<...>`, so enabling it
    // ~keep makes the renderer manufacture links those examples assert the absence of.
    options.render.r#unsafe = true;
    comrak::markdown_to_html(md, &options)
}

/// Escaping fully enabled: the configuration in which round-trip stability is a real contract.
///
/// ~keep With the shipped defaults, escaping is OFF, so text that merely looks like Markdown
/// ~keep is emitted verbatim: `<p>Foo\n***</p>` becomes `Foo\n***`, which re-parses as a
/// ~keep thematic break, and `<p>foo\n# bar</p>` re-parses as a heading. That is the defaults
/// ~keep trading round-trip safety for cleaner output, not a converter defect -- every such
/// ~keep case is corrected by `escape_misc` / `escape_asterisks`. Asserting on the escaping
/// ~keep configuration keeps this oracle measuring the converter rather than re-measuring a
/// ~keep product decision.
fn escaping_options() -> ConversionOptions {
    ConversionOptions {
        escape_misc: true,
        escape_asterisks: true,
        escape_underscores: true,
        ..Default::default()
    }
}

/// Floor for examples that must reach a fixpoint with escaping enabled.
///
/// ~keep A ratchet set to the measured value: 642 of 652 with escaping on, against 628 with
/// ~keep the shipped defaults. Raise it as more are fixed; a drop is a regression.
///
/// ~keep The 10 that remain fall into three groups.
///
/// ~keep Inherent to the round trip (252, 301, 302): two adjacent block quotes, or two
/// ~keep adjacent lists using the same bullet, have no separator in the source that survives
/// ~keep a compliant reparse -- they merge into one block whatever we emit. Emitting an
/// ~keep unrequested separator to force a fixpoint would corrupt the far more common case.
///
/// ~keep Renderer re-encoding we deliberately do not mirror (21, 344, 631): these carry
/// ~keep ASCII characters (backslash, backtick) that a compliant renderer percent-encodes in
/// ~keep a destination. Our own escaping already round-trips the content losslessly -- the
/// ~keep second and third conversions agree -- so only the byte spelling differs. Matching it
/// ~keep would mean reimplementing the renderer's full safe-character set as our default and
/// ~keep would disturb the documented backslash tradeoff in `inline/link.rs`.
///
/// ~keep Known open defects (175, 182, 642, 643): 642/643 are a real bug, not an artifact --
/// ~keep `normalize_link_label` collapses a hard line break inside a link label, so a `<br>`
/// ~keep there is lost on the second conversion. It is left alone deliberately: several
/// ~keep `~keep` comments in `tier1/scanner.rs` depend on the current behaviour for parity,
/// ~keep so it needs a coordinated two-tier change rather than a local edit.
const MIN_STABLE_ESCAPED: usize = 642;

#[test]
fn commonmark_spec_examples_reach_a_conversion_fixpoint() {
    let examples = spec_examples();
    assert!(
        examples.len() > 600,
        "spec fixture looks truncated: {} examples",
        examples.len()
    );

    let options = escaping_options();
    let mut stable = 0usize;
    let mut unstable: BTreeMap<String, Vec<u32>> = BTreeMap::new();

    for example in &examples {
        let Some(md1) = to_markdown(&example.html, &options) else {
            continue;
        };
        let Some(md2) = to_markdown(&render(&md1), &options) else {
            unstable
                .entry(example.section.clone())
                .or_default()
                .push(example.example);
            continue;
        };
        if md1 == md2 {
            stable += 1;
        } else {
            unstable
                .entry(example.section.clone())
                .or_default()
                .push(example.example);
        }
    }

    // ~keep Reported alongside, never asserted on: it measures how lossy the shipped
    // ~keep defaults are, which is a product decision rather than a regression signal.
    let default_stable = examples
        .iter()
        .filter(|e| {
            let defaults = ConversionOptions::default();
            to_markdown(&e.html, &defaults)
                .is_some_and(|md1| to_markdown(&render(&md1), &defaults).is_some_and(|md2| md1 == md2))
        })
        .count();

    println!("=== CommonMark spec fixpoint ===");
    println!(
        "default options (informational): {default_stable}/{} stable",
        examples.len()
    );
    println!("examples: {}", examples.len());
    println!("stable:   {stable}");
    println!("unstable: {}", examples.len() - stable);
    for (section, ids) in &unstable {
        println!("  {section}: {} ({:?})", ids.len(), &ids[..ids.len().min(8)]);
    }

    assert!(
        stable >= MIN_STABLE_ESCAPED,
        "only {stable} of {} spec examples reached a fixpoint with escaping enabled \
         (floor {MIN_STABLE_ESCAPED}); see the per-section breakdown above",
        examples.len()
    );
}

#[test]
fn commonmark_spec_examples_survive_both_newline_styles() {
    // ~keep The `<br>` defects fixed recently clustered in `newline_style`, and the spec's
    // ~keep hard-break and soft-break sections are exactly the inputs that exercise it. The
    // ~keep oracle here is only "does not panic and produces output", which is checkable for
    // ~keep every example without deciding a canonical form.
    let examples = spec_examples();
    for style in [NewlineStyle::Spaces, NewlineStyle::Backslash] {
        let options = ConversionOptions {
            newline_style: style,
            ..Default::default()
        };
        let mut converted = 0usize;
        for example in &examples {
            if to_markdown(&example.html, &options).is_some() {
                converted += 1;
            }
        }
        assert_eq!(
            converted,
            examples.len(),
            "{style:?}: only {converted} of {} spec examples converted",
            examples.len()
        );
    }
}
