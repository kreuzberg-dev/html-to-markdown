//! Hidden-element detection must read attribute *structure*, not raw tag text.
//!
//! Each of these reproduced a real defect before the fix: a substring scan matched the word
//! `hidden` inside a quoted value and deleted visible content; a CSS comment before the
//! property name defeated `display:none` detection and leaked hidden content; and `.any()`
//! over declarations ignored the cascade, so an overridden `display:none` still stripped.

use html_to_markdown_rs::{ConversionOptions, convert};

fn out(html: &str) -> String {
    convert(html, Some(ConversionOptions::default()))
        .expect("conversion failed")
        .content
        .unwrap_or_default()
}

#[test]
fn the_word_hidden_inside_an_attribute_value_is_not_a_hidden_attribute() {
    let got = out(r#"<div title="This section is hidden from search engines">Visible content</div>"#);
    assert!(
        got.contains("Visible content"),
        "`hidden` appearing inside a quoted attribute value must not strip the element; got {got:?}"
    );
}

#[test]
fn a_real_hidden_attribute_still_strips_the_element() {
    let got = out(r"<div hidden>secret</div><p>kept</p>");
    assert!(
        !got.contains("secret"),
        "a real `hidden` attribute must still strip; got {got:?}"
    );
    assert!(got.contains("kept"), "the sibling must survive; got {got:?}");
}

#[test]
fn a_css_comment_before_the_property_does_not_defeat_display_none() {
    let got = out(r#"<div style="/* note */ display:none">SECRET</div><p>visible</p>"#);
    assert!(
        !got.contains("SECRET"),
        "a CSS comment must not let hidden content leak into the output; got {got:?}"
    );
    assert!(got.contains("visible"), "the sibling must survive; got {got:?}");
}

#[test]
fn the_last_declaration_wins_so_an_overridden_display_none_stays_visible() {
    let got = out(r#"<div style="display:none; display:block">VISIBLE</div>"#);
    assert!(
        got.contains("VISIBLE"),
        "CSS cascade: the last declaration for a property wins, so this element is visible; got {got:?}"
    );
}

#[test]
fn an_unoverridden_display_none_among_several_declarations_still_strips() {
    let got = out(r#"<div style="color:red; display:none; margin:0">secret</div><p>kept</p>"#);
    assert!(
        !got.contains("secret"),
        "display:none still hides when not overridden; got {got:?}"
    );
    assert!(got.contains("kept"), "the sibling must survive; got {got:?}");
}

#[test]
fn visibility_hidden_is_honoured_and_overridable() {
    let hidden = out(r#"<div style="visibility:hidden">secret</div><p>kept</p>"#);
    assert!(
        !hidden.contains("secret"),
        "visibility:hidden must strip; got {hidden:?}"
    );

    let overridden = out(r#"<div style="visibility:hidden; visibility:visible">VISIBLE</div>"#);
    assert!(
        overridden.contains("VISIBLE"),
        "an overridden visibility:hidden is visible; got {overridden:?}"
    );
}
