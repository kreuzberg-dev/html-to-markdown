// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for issue #464: consecutive and trailing `<br>` were mishandled under
//! `newline_style="backslash"`. Two distinct defects were involved:
//!
//! - A run of N consecutive `<br>` collapsed to a single hard break plus a stray blank
//!   line instead of N hard breaks. `line_break.rs`'s dispatch treated "the previous
//!   sibling already emitted a break" (`output.ends_with('\n')`) the same as "nothing
//!   precedes this `<br>` on the current line" — so the second and later `<br>` in a run
//!   silently no-opped instead of emitting their own marker.
//! - A trailing run of `<br>` at the end of a block left literal, visible backslash
//!   characters behind. `CommonMark`'s hard-break syntax has no effect at the end of a
//!   block (<https://spec.commonmark.org/spec#hard-line-breaks>), but unlike the
//!   two-space style's leftover marker (invisible trailing whitespace), a leftover `\`
//!   is not whitespace, so later normalization passes could not clean it up.
//!
//! Both defects apply equally to the two-space style's *mid-content* run case; the
//! two-space style's *trailing* case was already correct (see
//! `should_keep_a_single_trailing_two_space_break`) because its leftover marker is inert.

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

fn convert(html: &str, options: ConversionOptions) -> String {
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

fn backslash_options() -> ConversionOptions {
    ConversionOptions {
        newline_style: NewlineStyle::Backslash,
        ..Default::default()
    }
}

// ~keep `newline_style: Spaces` is the default, and this markup has nothing else that
// ~keep forces Tier-2 (see `tier1/router.rs`), so it would otherwise run through the
// ~keep Tier-1 byte scanner — a separate implementation from the `line_break.rs` /
// ~keep `paragraph.rs` code this fix touches. `debug: true` is the router's documented
// ~keep Tier-2-forcing gate and does not itself affect rendered output, so it pins these
// ~keep cases onto the exact code path this fix changes.
fn spaces_options_on_tier2() -> ConversionOptions {
    ConversionOptions {
        newline_style: NewlineStyle::Spaces,
        debug: true,
        ..Default::default()
    }
}

// ~keep ── The three reporter cases, verbatim (backslash style) ────────────────────────

#[test]
fn should_emit_one_backslash_break_per_br_in_a_two_br_run() {
    assert_eq!(convert("<p>A<br/><br/>B</p>", backslash_options()), "A\\\n\\\nB\n");
}

#[test]
fn should_emit_one_backslash_break_per_br_in_a_three_br_run() {
    assert_eq!(
        convert("<p>A<br/><br/><br/>B</p>", backslash_options()),
        "A\\\n\\\n\\\nB\n"
    );
}

#[test]
fn should_drop_a_trailing_br_run_entirely() {
    assert_eq!(convert("<p>A<br/><br/><br/></p>", backslash_options()), "A\n");
}

// ~keep ── Equivalent cases, two-space style ───────────────────────────────────────────

#[test]
fn a_two_br_run_collapses_under_the_two_space_style_because_a_blank_line_cannot_carry_a_marker() {
    // ~keep NOT the same expectation as the backslash case above, and deliberately so. The
    // ~keep second break's marker would have to live on an otherwise-empty line, and this
    // ~keep crate strips trailing whitespace from blank lines by design -- see the locked
    // ~keep `whitespace_only_output_test.rs::should_keep_hard_break_after_content_but_not_on_a_blank_line`.
    // ~keep So the two-space style CANNOT express consecutive hard breaks; that limitation is
    // ~keep exactly why the backslash style exists and why issue #464 was reported against it.
    // ~keep Pinned here so a future change to the blank-line rule shows up as this test failing.
    assert_eq!(convert("<p>A<br/><br/>B</p>", spaces_options_on_tier2()), "A  \n\nB\n");
}

#[test]
fn a_three_br_run_collapses_the_same_way_under_the_two_space_style() {
    // ~keep Byte-identical to the two-<br> case: every marker after the first would need a
    // ~keep blank line to sit on. See the two-<br> test above for why that is by design.
    assert_eq!(
        convert("<p>A<br/><br/><br/>B</p>", spaces_options_on_tier2()),
        "A  \n\nB\n"
    );
}

#[test]
fn should_keep_a_single_trailing_two_space_break() {
    // ~keep Unlike the backslash style, a trailing "  \n" is invisible whitespace rather
    // ~keep than a visible artifact, so a trailing run is left with the same single
    // ~keep marker a lone trailing <br> already produces (matches
    // ~keep `whitespace_only_output_test.rs`'s "Only  \n") instead of being dropped.
    assert_eq!(convert("<p>A<br/><br/><br/></p>", spaces_options_on_tier2()), "A  \n");
}

// ~keep ── Boundary cases a naive per-style fix could still get wrong ──────────────────

#[test]
fn should_keep_a_single_mid_paragraph_br_working() {
    // ~keep Regression guard: a lone <br> with real content on both sides must still
    // ~keep emit exactly one break.
    assert_eq!(convert("<p>A<br/>B</p>", backslash_options()), "A\\\nB\n");
}

#[test]
fn should_emit_a_break_for_each_run_when_runs_are_separated_by_text() {
    assert_eq!(
        convert("<p>A<br/><br/>B<br/>C</p>", backslash_options()),
        "A\\\n\\\nB\\\nC\n"
    );
}

#[test]
fn a_leading_br_opens_a_line_and_the_rest_of_the_run_breaks_normally() {
    // ~keep NOT symmetric with the trailing-run case, and deliberately left that way.
    // ~keep Issue #464 is about CONSECUTIVE and TRAILING breaks; leading runs were never
    // ~keep reported. The first <br> of a leading run emits a bare newline because
    // ~keep `integration_test.rs::test_breaks_and_newlines_issue_112` pins exactly that for
    // ~keep a top-level <br>, and collapsing leading runs here would silently break it.
    // ~keep Every later <br> in the run has a line to break and emits a real marker.
    // ~keep Pinned so a future decision to collapse leading runs is a deliberate change
    // ~keep with this test updated, not an accident.
    assert_eq!(convert("<p><br/><br/>A</p>", backslash_options()), "\n\\\nA\n");
}

#[test]
fn should_produce_no_output_when_a_block_is_only_a_br_run() {
    // ~keep Nothing precedes and nothing follows the run: the leading-run rule empties
    // ~keep the block before any content is ever appended, so the block contributes no
    // ~keep paragraph separator either.
    assert_eq!(convert("<p><br/><br/></p>", backslash_options()), "");
}
