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
/// ~keep A ratchet set to the measured value: 644 of 652 with escaping on. Raise it as more
/// ~keep are fixed; a drop is a regression.
///
/// ~keep The 8 that remain fall into two groups.
///
/// ~keep Inherent to the round trip (252, 301, 302): two adjacent block quotes, or two
/// ~keep adjacent lists using the same bullet, have no separator in the source that survives
/// ~keep a compliant reparse -- they merge into one block whatever we emit. Emitting an
/// ~keep unrequested separator to force a fixpoint would corrupt the far more common case.
///
/// ~keep Renderer re-encoding we deliberately do not mirror (21, 344, 631, 642, 643): each was
/// ~keep measured (byte-for-byte, across two render/convert cycles, not assumed from a shared
/// ~keep label) to place an ASCII character in a link destination -- a backslash, a backtick,
/// ~keep or a raw space -- that a compliant renderer percent-encodes on its way back to HTML.
/// ~keep All five reach a fixpoint on the *second* cycle (`md2 == md3`), so none of them drift
/// ~keep or corrupt further; they only fail this test's stricter one-cycle (`md1 == md2`)
/// ~keep check. Matching the renderer's choice on the first cycle would mean reimplementing its
/// ~keep full safe-character set as our default and would disturb the documented backslash
/// ~keep tradeoff in `inline/link.rs`. Note that 642/643 also exercised a genuine hard-break
/// ~keep defect in `normalize_link_label`, which is fixed -- they reach this same
/// ~keep second-cycle-stable state only once the destination is clean, and remain listed here
/// ~keep only for the encoding difference.
///
/// ~keep 631 was, until it was fixed, a different and genuinely lossy defect wearing the same
/// ~keep label: `href="\*"` has a `\` immediately followed by ASCII punctuation in a *balanced*
/// ~keep destination, and `append_url_destination` left it unescaped -- a compliant reparse
/// ~keep consumes the `\` as a `CommonMark` escape of `*`, permanently dropping a byte the source
/// ~keep document had. That is not a byte-spelling difference; percent-encoding was never
/// ~keep involved in producing it. `escape_ambiguous_destination_backslashes` in `inline/link.rs`
/// ~keep now doubles a `\` there whenever the following byte is ASCII punctuation. 631 still
/// ~keep appears in the list above because, once its destination is correctly escaped, it
/// ~keep degrades into exactly the same one-cycle renderer re-encoding case as 21 -- confirmed
/// ~keep by the same `md1 != md2, md2 == md3` measurement.
///
/// ~keep A `\` that is the *last* character of `dest` was originally left unescaped
/// ~keep unconditionally, on the premise (the original audit #24 finding) that this call could
/// ~keep not know the byte `output` gains once the destination ends. That premise stopped being
/// ~keep true once `escape_ambiguous_destination_backslashes` gained a `title_follows` flag:
/// ~keep `append_markdown_link` and the image/graphic handlers always know, before calling
/// ~keep this function, whether a title is about to follow, so the next byte is either a
/// ~keep literal space (before a title -- not ASCII punctuation, so the `\` is already safe and
/// ~keep stays unescaped, preserving the case
/// ~keep `append_markdown_link_escapes_a_trailing_backslash_in_default_title_...` pins) or the
/// ~keep closing `)` (no title -- ASCII punctuation, so the `\` must be doubled). Leaving it
/// ~keep unescaped in the no-title case was worse than 631: `href="x\"` with no title emitted
/// ~keep `(x\)`, whose trailing `\)` reparses as an escaped paren, so the destination never
/// ~keep closes and the whole link -- not just one byte of it -- degrades to literal bracket
/// ~keep text. This is now fixed and does not affect `MIN_STABLE_ESCAPED`: no `CommonMark` spec
/// ~keep example exercises a no-title link whose destination ends in `\`.
const MIN_STABLE_ESCAPED: usize = 644;

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
