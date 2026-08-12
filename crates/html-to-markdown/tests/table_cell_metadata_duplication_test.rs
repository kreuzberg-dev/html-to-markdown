//! Regression coverage: the table column-width pre-pass must not be observable in the
//! public conversion result.
//!
//! Before `3a7c115a4` the pre-pass shared the conversion context's `Rc` metadata collector,
//! so every element inside a `<td>` was recorded twice — once by the width pre-pass and once
//! by the render pass. See `should_record_table_cell_link_metadata_exactly_once`.

#![cfg(feature = "metadata")]

use html_to_markdown_rs::{ConversionOptions, ConversionResult, DocumentNode, LinkStyle, NodeContent, convert};

const SINGLE_CELL_LINK_HTML: &str = r#"<table><tr><td><a href="https://example.com">L</a></td></tr></table>"#;
const SINGLE_CELL_IMAGE_HTML: &str = r#"<table><tr><td><img src="photo.png" alt="a photo"></td></tr></table>"#;

fn options_with_metadata() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: true,
        ..ConversionOptions::default()
    }
}

fn options_with_document_structure() -> ConversionOptions {
    ConversionOptions {
        include_document_structure: true,
        ..options_with_metadata()
    }
}

fn convert_with(html: &str, options: ConversionOptions) -> html_to_markdown_rs::metadata::HtmlMetadata {
    convert(html, Some(options)).expect("conversion must succeed").metadata
}

fn convert_to_result(html: &str, options: ConversionOptions) -> ConversionResult {
    convert(html, Some(options)).expect("conversion must succeed")
}

fn document_nodes(result: &ConversionResult) -> &[DocumentNode] {
    &result
        .document
        .as_ref()
        .expect("include_document_structure must populate result.document")
        .nodes
}

fn count_nodes(nodes: &[DocumentNode], predicate: fn(&NodeContent) -> bool) -> usize {
    nodes.iter().filter(|node| predicate(&node.content)).count()
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

// ~keep The structure collector is subject to the same "exactly one walk records" rule as the
// ~keep metadata collector, but it is not duplicated uniformly: `push_paragraph`, `push_heading`,
// ~keep `push_list_start`/`push_list_item` are all gated on `!ctx.in_table_cell` and every cell
// ~keep walk sets that flag, so those node kinds are never recorded from a cell at all. Only the
// ~keep ungated sites — `push_image`, `push_code` and a nested table's `push_table_data` — were
// ~keep recorded once per walk. Both facts are pinned below.

#[test]
fn should_record_table_cell_image_in_document_structure_exactly_once() {
    let result = convert_to_result(SINGLE_CELL_IMAGE_HTML, options_with_document_structure());
    let nodes = document_nodes(&result);

    assert_eq!(
        count_nodes(nodes, |content| matches!(content, NodeContent::Image { .. })),
        1,
        "the width pre-pass, the render pass and the TableGrid walk all traverse the cell; only \
         one of them may push the image"
    );
    assert_eq!(
        count_nodes(nodes, |content| matches!(content, NodeContent::Table { .. })),
        1,
        "one <table> must yield one table node"
    );
    assert_eq!(nodes.len(), 2, "the document holds exactly the image and the table");

    let image = nodes.iter().find_map(|node| match &node.content {
        NodeContent::Image { src, description, .. } => Some((src.as_deref(), description.as_deref())),
        _ => None,
    });
    assert_eq!(image, Some((Some("photo.png"), Some("a photo"))));

    assert_eq!(
        result.metadata.images.len(),
        1,
        "the metadata and structure collectors must agree on how many images the cell holds"
    );
}

#[test]
fn should_record_table_cell_code_block_in_document_structure_exactly_once() {
    let html = "<table><tr><td><pre><code>let x = 1;</code></pre></td></tr></table>";
    let result = convert_to_result(html, options_with_document_structure());
    let nodes = document_nodes(&result);

    assert_eq!(
        count_nodes(nodes, |content| matches!(content, NodeContent::Code { .. })),
        1,
        "push_code is not gated on in_table_cell, so every walk of the cell recorded the block"
    );
    assert_eq!(
        nodes.len(),
        2,
        "the document holds exactly the code block and the table"
    );
}

#[test]
fn should_record_a_nested_table_in_document_structure_exactly_once() {
    let html = "<table><tr><td><table><tr><td>inner</td></tr></table></td></tr></table>";
    let result = convert_to_result(html, options_with_document_structure());
    let nodes = document_nodes(&result);

    assert_eq!(
        count_nodes(nodes, |content| matches!(content, NodeContent::Table { .. })),
        2,
        "the outer table and the one nested table — the TableGrid walk must not re-dispatch the \
         nested table into the collector"
    );
    assert_eq!(
        result.tables.len(),
        2,
        "result.tables is filled by the same push_table_data call as the table nodes"
    );
}

// ~keep This pins the negative half of the rule: a <p> in a <td> is not recorded 3x, it is
// ~keep recorded 0x, because `paragraph.rs` skips `push_paragraph` when `in_table_cell` is set.
// ~keep The cell's text reaches the caller through the table node's `TableGrid` instead. The
// ~keep assertion is what makes a future "fix" that starts emitting cell paragraphs visible.
#[test]
fn should_not_record_a_paragraph_inside_a_table_cell_in_document_structure() {
    let result = convert_to_result(
        "<table><tr><td><p>Cell body</p></td></tr></table>",
        options_with_document_structure(),
    );
    let nodes = document_nodes(&result);

    assert_eq!(
        count_nodes(nodes, |content| matches!(content, NodeContent::Paragraph { .. })),
        0,
        "cell text belongs to the table node's grid, not to a standalone paragraph node"
    );
    assert_eq!(nodes.len(), 1, "the table node is the document's only node");
    assert!(
        matches!(nodes[0].content, NodeContent::Table { .. }),
        "the single node must be the table, got {:?}",
        nodes[0].content
    );
}

#[cfg(feature = "inline-images")]
const ONE_PIXEL_PNG_DATA_URI: &str = concat!(
    "data:image/png;base64,",
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
);

#[cfg(feature = "inline-images")]
fn single_cell_inline_image_html() -> String {
    format!(r#"<table><tr><td><img src="{ONE_PIXEL_PNG_DATA_URI}" alt="pixel"></td></tr></table>"#)
}

#[cfg(feature = "inline-images")]
#[test]
fn should_collect_a_table_cell_inline_image_exactly_once() {
    let result = convert_to_result(
        &single_cell_inline_image_html(),
        ConversionOptions {
            extract_images: true,
            ..options_with_metadata()
        },
    );

    assert_eq!(
        result.images.len(),
        1,
        "the inline collector disables cell-text reuse, so the pre-pass and the render pass both \
         walked the cell; only the render pass may collect"
    );
    assert_eq!(result.images[0].description.as_deref(), Some("pixel"));
    assert_eq!(
        result.images[0].filename.as_deref(),
        Some("embedded_image_1.png"),
        "the generated filename carries the collector index, so a discarded walk must not consume one"
    );
}

#[cfg(feature = "inline-images")]
#[test]
fn should_collect_a_table_cell_inline_image_once_when_document_structure_is_enabled() {
    let result = convert_to_result(
        &single_cell_inline_image_html(),
        ConversionOptions {
            extract_images: true,
            ..options_with_document_structure()
        },
    );

    assert_eq!(
        result.images.len(),
        1,
        "include_document_structure adds the TableGrid walk as a third traversal of the cell"
    );
    assert_eq!(
        count_nodes(document_nodes(&result), |content| matches!(
            content,
            NodeContent::Image { .. }
        )),
        1,
        "the structure and inline-image collectors must agree on how many images the cell holds"
    );
}
