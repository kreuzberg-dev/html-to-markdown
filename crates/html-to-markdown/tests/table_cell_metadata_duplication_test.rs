//! Regression coverage: the table column-width pre-pass must not be observable in the
//! public conversion result.
//!
//! Before `3a7c115a4` the pre-pass shared the conversion context's `Rc` metadata collector,
//! so every element inside a `<td>` was recorded twice — once by the width pre-pass and once
//! by the render pass. See `should_record_table_cell_link_metadata_exactly_once`.

#![cfg(feature = "metadata")]

use html_to_markdown_rs::{ConversionOptions, LinkStyle, convert};

const SINGLE_CELL_LINK_HTML: &str = r#"<table><tr><td><a href="https://example.com">L</a></td></tr></table>"#;

fn options_with_metadata() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: true,
        ..ConversionOptions::default()
    }
}

fn convert_with(html: &str, options: ConversionOptions) -> html_to_markdown_rs::metadata::HtmlMetadata {
    convert(html, Some(options)).expect("conversion must succeed").metadata
}

#[test]
fn should_record_table_cell_link_metadata_exactly_once() {
    let metadata = convert_with(SINGLE_CELL_LINK_HTML, options_with_metadata());

    assert_eq!(metadata.links.len(), 1, "a single <a> in a <td> must be recorded once");
    assert_eq!(metadata.links[0].href, "https://example.com");
    assert_eq!(metadata.links[0].text, "L");
}

#[test]
fn should_record_link_metadata_once_outside_a_table() {
    let metadata = convert_with(r#"<p><a href="https://example.com">L</a></p>"#, options_with_metadata());

    assert_eq!(metadata.links.len(), 1);
    assert_eq!(metadata.links[0].href, "https://example.com");
}

#[test]
fn should_record_each_link_once_when_a_row_has_several_cells() {
    let html = r#"<table><tr><td><a href="https://a.example">A</a></td><td><a href="https://b.example">B</a></td></tr></table>"#;
    let metadata = convert_with(html, options_with_metadata());

    assert_eq!(
        metadata.links.len(),
        2,
        "two cells with one link each must yield two links"
    );
    assert_eq!(metadata.links[0].href, "https://a.example");
    assert_eq!(metadata.links[1].href, "https://b.example");
}

#[test]
fn should_record_nested_table_cell_link_metadata_exactly_once() {
    let html = concat!(
        "<table><tr><td><table><tr><td>",
        r#"<a href="https://example.com">L</a>"#,
        "</td></tr></table></td></tr></table>"
    );
    let metadata = convert_with(html, options_with_metadata());

    assert_eq!(
        metadata.links.len(),
        1,
        "a link in a nested table cell must be recorded once"
    );
    assert_eq!(metadata.links[0].href, "https://example.com");
}

#[test]
fn should_record_table_cell_image_metadata_exactly_once() {
    let html = r#"<table><tr><td><img src="photo.png" alt="a photo"></td></tr></table>"#;
    let metadata = convert_with(html, options_with_metadata());

    assert_eq!(
        metadata.images.len(),
        1,
        "a single <img> in a <td> must be recorded once"
    );
    assert_eq!(metadata.images[0].src, "photo.png");
    assert_eq!(metadata.images[0].alt.as_deref(), Some("a photo"));
}

// ~keep The three tests below pin the invariant across the option combinations that disable
// ~keep cell-text reuse: a cell's metadata is recorded by exactly one walk regardless of how many
// ~keep times the cell is traversed. With reuse on the width pre-pass is the recording walk; with
// ~keep reuse off the render pass is, and the pre-pass plus the structure collector's TableGrid
// ~keep walk run with the metadata collector detached. Counts here are option-independent — they
// ~keep must match the same HTML converted with the default options.

#[test]
fn should_record_table_cell_link_once_when_document_structure_is_enabled() {
    let metadata = convert_with(
        SINGLE_CELL_LINK_HTML,
        ConversionOptions {
            include_document_structure: true,
            ..options_with_metadata()
        },
    );

    assert_eq!(
        metadata.links.len(),
        1,
        "include_document_structure adds a width pre-pass walk and a TableGrid walk on top of \
         the render pass; only the render pass may record"
    );
}

#[test]
fn should_record_link_once_outside_a_table_when_document_structure_is_enabled() {
    let metadata = convert_with(
        r#"<p><a href="https://example.com">L</a></p>"#,
        ConversionOptions {
            include_document_structure: true,
            ..options_with_metadata()
        },
    );

    assert_eq!(
        metadata.links.len(),
        1,
        "a link outside a table is walked once in every configuration"
    );
}

#[test]
fn should_record_table_cell_link_once_when_reference_link_style_is_used() {
    let metadata = convert_with(
        SINGLE_CELL_LINK_HTML,
        ConversionOptions {
            link_style: LinkStyle::Reference,
            ..options_with_metadata()
        },
    );

    assert_eq!(
        metadata.links.len(),
        1,
        "the reference collector disables cell-text reuse, so the render pass walks the cell and \
         the width pre-pass must not record"
    );
}
