// ~keep The inner attribute below is a crate-level Rust attribute, not a shell shebang.
#![allow(missing_docs)]

//! `strip_hidden_elements` must only treat a real tag open as a tag.
//!
//! HTML5's tag-open state only begins a tag name when `<` is followed by an ASCII letter
//! (<https://html.spec.whatwg.org/#tag-open-state>); `<1div>`, `<_div>`, `<-div>`, `< div>`
//! and `<9 …>` are ordinary text. The hidden-element pre-pass used to accept any `<` whose
//! next byte was not `/` or `!`, scan forward for the next `>` anywhere in the document, and
//! treat whatever it spanned as a tag. That produced two distinct defects, both fixed by
//! requiring a real tag-name start:
//!
//! - **Visible text was silently deleted.** `<1div hidden>x</1div>` is entirely text, but the
//!   pre-pass matched it as a hidden element and removed the whole span, including the `x`.
//! - **Genuinely hidden content leaked.** In `<<div hidden>x</div>` the span began at the
//!   stray first `<`, so the slice that got removed was `<<div hidden>` and the `x` inside the
//!   real hidden `<div>` survived into the output.
//!
//! The same change also removed a quadratic blow-up; that aspect is pinned separately by
//! `bare_lt_complexity.rs`.

use html_to_markdown_rs::options::ConversionOptions;

fn convert(html: &str) -> String {
    html_to_markdown_rs::convert(html, Some(ConversionOptions::default()))
        .expect("conversion should succeed")
        .content
        .unwrap_or_default()
}

#[test]
fn should_not_delete_text_that_merely_looks_like_a_hidden_tag() {
    for (html, expected) in [
        ("<1div hidden>x</1div>", "<1div hidden>x</1div>\n"),
        ("<_div hidden>x</_div>", "<_div hidden>x</_div>\n"),
        ("<-div hidden>x", "<-div hidden>x\n"),
        ("< div hidden>x", "< div hidden>x\n"),
        ("<9 hidden>y", "<9 hidden>y\n"),
    ] {
        assert_eq!(convert(html), expected, "input: {html}");
    }
}

#[test]
fn should_not_let_hidden_change_the_rendering_of_a_non_tag() {
    // ~keep The invariant behind the case above, stated so it cannot be satisfied by
    // ~keep hard-coding those five strings: `hidden` is an attribute, attributes only exist
    // ~keep inside tags, so on something that is not a tag it must be inert. Comparing the
    // ~keep two renderings catches any future pre-pass that starts special-casing the word.
    for (hidden, inert) in [
        ("<1div hidden>x</1div>", "<1div data-q>x</1div>"),
        ("<_div hidden>x</_div>", "<_div data-q>x</_div>"),
        ("<-div hidden>x", "<-div data-q>x"),
        ("< div hidden>x", "< div data-q>x"),
    ] {
        assert_eq!(
            convert(hidden),
            convert(inert).replace("data-q", "hidden"),
            "`hidden` changed the rendering of a non-tag: {hidden}"
        );
    }
}

#[test]
fn should_still_strip_a_real_hidden_element() {
    // ~keep The counterweight: narrowing what counts as a tag open must not stop the pre-pass
    // ~keep doing its job.
    assert_eq!(convert("<div hidden>x</div>"), "");
    assert_eq!(convert(r#"<div style="display:none">x</div>"#), "");
    assert_eq!(convert(r#"<div style="visibility:hidden">x</div>"#), "");
    assert_eq!(convert("<DIV HIDDEN>x</DIV>"), "");
}

#[test]
fn should_strip_a_hidden_element_that_follows_a_stray_open_angle_bracket() {
    // ~keep Was `"x\n"`: the removed span started at the stray `<` and ended at the first
    // ~keep `>`, so it deleted `<<div hidden>` and left the hidden `x` visible. The stray `<`
    // ~keep is text and must survive; the hidden element must not.
    assert_eq!(convert("<<div hidden>x</div>"), "<\n");
}

#[test]
fn should_strip_a_hidden_element_that_follows_unrelated_markup() {
    assert_eq!(convert("<!doctype html><div hidden>x</div>"), "");
    assert_eq!(convert("<9 hidden><div hidden>y</div>"), "<9 hidden>\n");

    // ~keep The hidden `<div>` is stripped here too -- but the `<?php ... ?>` ahead of it
    // ~keep leaks its own text. HTML5 puts `<?` into the bogus-comment state, which consumes
    // ~keep through the next `>` and produces a comment node, so this should render as
    // ~keep nothing at all. That leak is unrelated to the tag-open-state fix (it is
    // ~keep byte-identical before and after it) and is pinned here as current behaviour
    // ~keep rather than as desired behaviour, so that fixing it shows up as this assertion
    // ~keep failing instead of going unnoticed.
    assert_eq!(convert("<?php echo 1; ?><div hidden>x</div>"), "?php echo 1; ?>\n");
}
