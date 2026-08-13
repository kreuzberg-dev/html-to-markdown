//! Table cell conversion utilities.
//!
//! Handles conversion of table cell (td/th) elements to Markdown format,
//! including colspan support and content normalization.

use std::borrow::Cow;

/// Maximum allowed table columns to prevent unbounded memory usage.
const MAX_TABLE_COLS: usize = 1000;

/// Get colspan attribute value from an element.
///
/// Reads the colspan attribute from a table cell, with bounds checking
/// to prevent memory exhaustion attacks.
///
/// # Arguments
/// * `node_handle` - Handle to the cell element
/// * `parser` - HTML parser instance
///
/// # Returns
/// The colspan value (minimum 1, maximum `MAX_TABLE_COLS`)
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_colspan(node_handle: &tl::NodeHandle, parser: &tl::Parser) -> usize {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        if let Some(Some(bytes)) = tag.attributes().get("colspan") {
            if let Ok(colspan) = bytes.as_utf8_str().parse::<usize>() {
                return clamp_table_span(colspan);
            }
        }
    }
    1
}

/// Get both colspan and rowspan in a single lookup.
///
/// More efficient than calling `get_colspan` and a separate rowspan lookup.
///
/// # Arguments
/// * `node_handle` - Handle to the cell element
/// * `parser` - HTML parser instance
///
/// # Returns
/// A tuple of (colspan, rowspan), both minimum 1 and maximum `MAX_TABLE_COLS`
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_colspan_rowspan(node_handle: &tl::NodeHandle, parser: &tl::Parser) -> (usize, usize) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let attrs = tag.attributes();
        let colspan = attrs
            .get("colspan")
            .flatten()
            .and_then(|v| v.as_utf8_str().parse::<usize>().ok())
            .map_or(1, clamp_table_span);
        let rowspan = attrs
            .get("rowspan")
            .flatten()
            .and_then(|v| v.as_utf8_str().parse::<usize>().ok())
            .map_or(1, clamp_table_span);
        (colspan, rowspan)
    } else {
        (1, 1)
    }
}

/// Clamp a table span value to safe bounds.
///
/// Prevents memory exhaustion by clamping colspan/rowspan values.
fn clamp_table_span(value: usize) -> usize {
    if value == 0 { 1 } else { value.min(MAX_TABLE_COLS) }
}

/// Collect table cells (td/th) from a row element.
///
/// Extracts only the direct cell children of a row, filtering by tag name.
///
/// # Arguments
/// * `node_handle` - Handle to the row element
/// * `parser` - HTML parser instance
/// * `dom_ctx` - DOM context for tag name resolution
/// * `cells` - Mutable vector to populate with cell handles
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn collect_table_cells(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &super::super::super::DomContext,
    cells: &mut Vec<tl::NodeHandle>,
) {
    cells.clear();
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let children = tag.children();
        for child_handle in children.top().iter() {
            if let Some(cell_name) = dom_ctx.tag_name_for(*child_handle, parser) {
                if matches!(cell_name.as_ref(), "th" | "td" | "cell") {
                    cells.push(*child_handle);
                }
            }
        }
    }
}

/// Extract the text content of a table cell for column width calculation.
///
/// Returns the same text that would appear in the rendered cell, without
/// the surrounding pipe delimiters. Used in the first pass to compute
/// maximum column widths before rendering with padding.
///
/// # Arguments
/// * `node_handle` - Handle to the cell element
/// * `parser` - HTML parser instance
/// * `options` - Conversion options
/// * `ctx` - Conversion context
/// * `dom_ctx` - DOM context
/// * `depth` - Current recursion depth (the cell's own depth; children are walked at `depth + 1`)
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn cell_text_content(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
) -> String {
    let cell_ctx = super::super::super::Context {
        in_table_cell: true,
        ..ctx.clone()
    };

    render_cell_text(node_handle, parser, options, &cell_ctx, dom_ctx, depth)
}

/// Initial buffer capacity for a rendered cell's markdown.
const CELL_TEXT_CAPACITY: usize = 128;

/// Render a cell's content to the exact text that appears between its pipe delimiters.
///
/// `cell_ctx` must already carry `in_table_cell = true`; the caller owns that decision so
/// the per-row context can be built once instead of cloned per cell.
///
/// `depth` is the cell's own recursion depth; children are walked at `depth + 1`.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn render_cell_text(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    cell_ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
) -> String {
    let mut text = String::with_capacity(CELL_TEXT_CAPACITY);

    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let children = tag.children();
        let has_tag_child = children
            .top()
            .iter()
            .any(|child_handle| matches!(child_handle.get(parser), Some(tl::Node::Tag(_))));

        if has_tag_child {
            for child_handle in children.top().iter() {
                super::super::super::walk_node(child_handle, parser, &mut text, options, cell_ctx, depth + 1, dom_ctx);
            }
        } else {
            let raw = dom_ctx.text_content(*node_handle, parser);
            let normalized = if options.whitespace_mode == crate::options::WhitespaceMode::Normalized {
                crate::text::normalize_cell_whitespace_cow(raw.as_str())
            } else {
                Cow::Borrowed(raw.as_str())
            };
            text = escape_cell_text(normalized.as_ref(), options);
        }
    }

    trim_in_place(&mut text);
    if !options.br_in_tables && text.contains('\n') {
        text = text.replace('\n', " ");
    }
    text
}

/// Trim leading and trailing whitespace without reallocating.
fn trim_in_place(text: &mut String) {
    let end = text.trim_end().len();
    text.truncate(end);
    let start = text.len() - text.trim_start().len();
    text.drain(..start);
}

/// Escape text for use inside a table cell.
///
/// Always escapes `*` and `_` (to prevent unintended emphasis inside cells),
/// applies `escape_misc` / `escape_ascii` per options, and escapes `|` (pipe)
/// when `escape_misc` is not already handling it.
fn escape_cell_text(text: &str, options: &crate::options::ConversionOptions) -> String {
    // ~keep Always escape * and _ in table cells to prevent unintended emphasis.
    let escaped = crate::text::escape(text, options.escape_misc, true, true, options.escape_ascii);
    if options.escape_misc {
        escaped.into_owned()
    } else {
        escaped.replace('|', r"\|")
    }
}

/// Convert a table cell (td or th) to Markdown format.
///
/// Processes cell content and renders it with pipe delimiters for Markdown tables.
/// Handles colspan by adding extra pipes, and escapes pipes in cell content.
/// Always escapes `*` and `_` to prevent unintended emphasis inside cells.
///
/// # Arguments
/// * `node_handle` - Handle to the cell element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append cell content
/// * `options` - Conversion options (escape settings, `br_in_tables`)
/// * `ctx` - Conversion context (visitor, etc)
/// * `_tag_name` - Tag name (for consistency, not used)
/// * `dom_ctx` - DOM context for content extraction
/// * `col_width` - Optional target width for padding (None = no padding)
/// * `depth` - Current recursion depth (the cell's own depth; children are walked at `depth + 1`)
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::too_many_arguments)]
pub fn convert_table_cell(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    cell_ctx: &super::super::super::Context,
    _tag_name: &str,
    dom_ctx: &super::super::super::DomContext,
    col_width: Option<usize>,
    depth: usize,
) {
    let text = render_cell_text(node_handle, parser, options, cell_ctx, dom_ctx, depth);
    emit_cell_text(node_handle, parser, output, &text, col_width);
}

/// Emit already-rendered cell text with its padding and colspan pipe delimiters.
///
/// Split out from [`convert_table_cell`] so a cell rendered during the column-width
/// pre-pass can be emitted without walking its subtree a second time.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn emit_cell_text(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    text: &str,
    col_width: Option<usize>,
) {
    let colspan = get_colspan(node_handle, parser);

    output.push(' ');
    output.push_str(text);
    if let Some(width) = col_width {
        let text_len = text.chars().count();
        if text_len < width {
            for _ in 0..(width - text_len) {
                output.push(' ');
            }
        }
    }
    for _ in 0..colspan {
        output.push_str(" |");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rich_formatting_preserved_in_cells() {
        let html = "<table><tr><th>H</th></tr><tr><td><strong>Bold</strong> and <em>italic</em></td></tr></table>";
        let result = crate::convert(html, None).unwrap();
        let content = result.content.unwrap_or_default();
        assert!(
            content.contains("**Bold**") || content.contains("__Bold__"),
            "bold should be preserved: {content}"
        );
        assert!(
            content.contains("*italic*") || content.contains("_italic_"),
            "italic should be preserved: {content}"
        );
    }
}
