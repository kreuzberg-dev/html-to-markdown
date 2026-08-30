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
/// ~keep A ratchet set to the measured value: 609 of 652 with escaping on, against 597 with
/// ~keep the defaults. Fixed since the previous 607 floor: an `<a>`/`<img>` with no visible
/// ~keep text falls back to its href as the label (`handlers/link.rs`); that fallback bypassed
/// ~keep the normal text escaper entirely, so a raw `*`/`_` in the href round-tripped into real
/// ~keep emphasis (476, 477). A related destination-escaping gap (`inline/link.rs`,
/// ~keep `append_url_destination`) was also closed -- a literal backslash preceding the paren
/// ~keep escaping in an unbalanced-parens destination was not itself escaped, and a raw line
/// ~keep ending inside an angle-bracket-wrapped destination is not valid `CommonMark` at all --
/// ~keep but every example that exercised it (21, 631, 642, 643) also happens to trip comrak's
/// ~keep link-destination percent-encoding on the very next byte, so it stays unstable for that
/// ~keep separate reason and the fix does not move this floor; it is still a real, verified
/// ~keep content-preservation fix (see the PR/commit description for the byte-level evidence).
/// ~keep A wider fix was investigated and deliberately not applied: the same leading-space
/// ~keep instability affecting most of the remaining HTML-blocks failures traces to
/// ~keep `text_node.rs`'s whitespace-collapsing (`skip_prefix`/the whitespace-only branch)
/// ~keep having no way to tell "start of the real document/block output" apart from "start of
/// ~keep a handler's private scratch buffer" (`emphasis.rs`, `typography.rs`'s sub/sup, and
/// ~keep every other inline wrapper build their content into a fresh local `String` before
/// ~keep splicing it into non-empty output) -- an `output.is_empty()` guard there fixed 16 of
/// ~keep the 45 (entity refs 25/40, fenced code 138, HTML blocks 150/151/155/161/163-166/171/173/
/// ~keep 184/186/189) but broke `test_subscript_leading_whitespace` / `test_superscript_leading_whitespace`
/// ~keep in `integration_test.rs` (and, per the same mechanism, silently changed output for
/// ~keep every inline wrapper whose first child starts with whitespace). It needs a real
/// ~keep "am I at document start" signal threaded through every scratch-buffer call site, not
/// ~keep a one-line guard -- left for follow-up. Raise this number as more are fixed; a drop is
/// ~keep a regression.
const MIN_STABLE_ESCAPED: usize = 609;

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
