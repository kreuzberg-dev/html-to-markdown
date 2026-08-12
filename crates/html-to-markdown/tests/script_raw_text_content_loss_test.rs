//! Reachability probe for the raw-text depth-counting defect.

use html_to_markdown_rs::{ConversionOptions, convert};

#[test]
fn a_literal_script_open_tag_inside_a_script_body_does_not_drop_the_document() {
    let html = r#"<script>var s = "<script>";</script><p>Visible paragraph</p>"#;
    let out = convert(html, Some(ConversionOptions::default())).expect("conversion failed");
    assert!(
        out.content.as_deref().unwrap_or_default().contains("Visible paragraph"),
        "content after a script body containing a literal `<script>` was dropped; got {out:?}"
    );
}

#[test]
fn a_literal_style_open_tag_inside_a_style_body_does_not_drop_the_document() {
    let html = r"<style>/* <style> */</style><p>Visible paragraph</p>";
    let out = convert(html, Some(ConversionOptions::default())).expect("conversion failed");
    assert!(
        out.content.as_deref().unwrap_or_default().contains("Visible paragraph"),
        "content after a style body containing a literal `<style>` was dropped; got {out:?}"
    );
}
