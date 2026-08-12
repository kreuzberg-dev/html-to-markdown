//! Cell and row handling for Markdown conversion.
//!
//! Provides functionality for processing table cells and rows, including:
//! - Row conversion to Markdown format
//! - Cell layout handling with colspan/rowspan support
//! - Layout table row conversion to list items

use crate::converter::utility::content::normalized_tag_name;
use std::borrow::Cow;
use std::collections::HashMap;

use super::cell::{cell_text_content, collect_table_cells, convert_table_cell, emit_cell_text, get_colspan_rowspan};

/// Maximum allowed table columns to prevent unbounded memory usage.
const MAX_TABLE_COLS: usize = 1000;

/// Rendered markdown of each cell visited by the column-width pre-pass, keyed by DOM node id.
///
/// ~keep The pre-pass and the render pass walk the same cell subtrees, so the pre-pass result
/// ~keep can be emitted directly instead of rendering twice — but only where the two passes are
/// ~keep provably byte-identical. The pre-pass context sets `measure_width_only` (nested tables
/// ~keep degrade to raw text, issue #406) and `skip_visitor_hooks`, and a second walk is
/// ~keep observable through any collector handle shared via the context. The caller decides via
/// ~keep `enabled` (see `builder::cell_text_reuse_allowed`); when disabled this stores nothing.
pub struct CellTextCache {
    entries: HashMap<u32, String>,
    enabled: bool,
}

impl CellTextCache {
    /// Create a cache; `enabled == false` makes every store a no-op and every lookup a miss.
    pub fn new(enabled: bool) -> Self {
        Self {
            entries: HashMap::new(),
            enabled,
        }
    }

    fn store(&mut self, node_id: u32, text: String) {
        if self.enabled {
            self.entries.insert(node_id, text);
        }
    }

    fn take(&mut self, node_id: u32) -> Option<String> {
        if self.enabled {
            self.entries.remove(&node_id)
        } else {
            None
        }
    }
}

/// Append a layout table row as a list item.
///
/// For tables used for visual layout, converts rows to list items
/// instead of table format for better readability.
///
/// # Arguments
/// * `row_handle` - Handle to the row element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append content
/// * `options` - Conversion options
/// * `ctx` - Conversion context
/// * `dom_ctx` - DOM context
/// * `depth` - Current recursion depth (the row's own depth; cell content is walked at `depth + 1`)
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::too_many_arguments)]
pub fn append_layout_row(
    row_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
) {
    if let Some(tl::Node::Tag(row_tag)) = row_handle.get(parser) {
        let mut row_text = String::new();
        let row_children = row_tag.children();
        for cell_handle in row_children.top().iter() {
            if let Some(tl::Node::Tag(cell_tag)) = cell_handle.get(parser) {
                let cell_name: Cow<'_, str> = dom_ctx.tag_info(cell_handle.get_inner(), parser).map_or_else(
                    || normalized_tag_name(cell_tag.name().as_utf8_str()).into_owned().into(),
                    |info| Cow::Borrowed(info.name.as_str()),
                );
                if matches!(cell_name.as_ref(), "td" | "th" | "cell") {
                    let mut cell_text = String::new();
                    // ~keep issue #433: honor keep_inline_images_in for images in
                    // ~keep layout-table cells even though cell content is converted
                    // ~keep as inline. A matching cell tag keeps images as markdown.
                    let cell_allow_inline_images = ctx.keep_inline_images_in.contains(cell_name.as_ref());
                    let cell_ctx = super::super::super::Context {
                        convert_as_inline: true,
                        cell_allow_inline_images,
                        ..ctx.clone()
                    };
                    let cell_children = cell_tag.children();
                    for cell_child in cell_children.top().iter() {
                        super::super::super::walk_node(
                            cell_child,
                            parser,
                            &mut cell_text,
                            options,
                            &cell_ctx,
                            depth + 1,
                            dom_ctx,
                        );
                    }
                    let cell_content = crate::text::normalize_whitespace_cow(&cell_text);
                    if !cell_content.trim().is_empty() {
                        if !row_text.is_empty() {
                            row_text.push(' ');
                        }
                        row_text.push_str(cell_content.trim());
                    }
                }
            }
        }

        let trimmed = row_text.trim();
        if !trimmed.is_empty() {
            if !output.is_empty() && !output.ends_with('\n') {
                output.push('\n');
            }
            let formatted = trimmed.strip_prefix("- ").unwrap_or(trimmed).trim_start();
            output.push_str("- ");
            output.push_str(formatted);
            output.push('\n');
        }
    }
}

/// Collect the rendered text content of every cell in a row for width calculation.
///
/// `rowspan_tracker` mirrors the tracker in `convert_table_row` so that spanned
/// columns are skipped in the width pre-pass just as they are skipped in rendering.
/// Pass a shared tracker across all row calls to correctly handle multi-row spans.
///
/// `depth` is the row's own recursion depth; cell content is measured at `depth + 1`.
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::too_many_arguments)]
pub fn collect_row_cell_widths(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    col_widths: &mut Vec<usize>,
    rowspan_tracker: &mut Vec<Option<usize>>,
    cell_cache: &mut CellTextCache,
    depth: usize,
) {
    let mut cells = Vec::new();
    collect_table_cells(node_handle, parser, dom_ctx, &mut cells);

    let mut col = 0usize;
    let mut cell_iter = cells.iter();

    loop {
        while col < rowspan_tracker.len() {
            if let Some(Some(remaining)) = rowspan_tracker.get_mut(col) {
                if *remaining > 0 {
                    *remaining -= 1;
                    if *remaining == 0 {
                        rowspan_tracker[col] = None;
                    }
                    col += 1;
                    continue;
                }
            }
            break;
        }

        let Some(cell_handle) = cell_iter.next() else {
            break;
        };

        let text = cell_text_content(cell_handle, parser, options, ctx, dom_ctx, depth + 1);
        const MAX_CELL_WIDTH: usize = 200;
        let width = text.chars().count().min(MAX_CELL_WIDTH);
        cell_cache.store(cell_handle.get_inner(), text);

        if col >= col_widths.len() {
            col_widths.resize(col + 1, 0);
        }
        if width > col_widths[col] {
            col_widths[col] = width;
        }

        let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

        if rowspan > 1 {
            if col >= rowspan_tracker.len() {
                rowspan_tracker.resize(col + 1, None);
            }
            rowspan_tracker[col] = Some(rowspan - 1);
        }

        col = col.saturating_add(colspan);
    }
}

/// Emit one cell of a rendered row, reusing the pre-pass rendering when it is cached.
///
/// `depth` is the cell's own recursion depth.
#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::too_many_arguments)]
fn emit_row_cell(
    cell_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    row_text: &mut String,
    options: &crate::options::ConversionOptions,
    cell_ctx: &super::super::super::Context,
    dom_ctx: &super::super::super::DomContext,
    col_width: Option<usize>,
    depth: usize,
    cell_cache: &mut CellTextCache,
) {
    if let Some(text) = cell_cache.take(cell_handle.get_inner()) {
        emit_cell_text(cell_handle, parser, row_text, &text, col_width);
    } else {
        convert_table_cell(
            cell_handle,
            parser,
            row_text,
            options,
            cell_ctx,
            "",
            dom_ctx,
            col_width,
            depth,
        );
    }
}

/// Minimum separator dash count per column (matches `---`).
const MIN_SEPARATOR_DASHES: usize = 3;

/// Convert a table row (tr) to Markdown format.
///
/// Processes all cells in a row, handling colspan and rowspan for proper
/// column alignment. Renders header separator row after the first row.
/// Integrates with visitor pattern for custom row handling.
///
/// # Arguments
/// * `node_handle` - Handle to the row element
/// * `parser` - HTML parser instance
/// * `output` - Mutable string to append row content
/// * `options` - Conversion options
/// * `ctx` - Conversion context (visitor, etc)
/// * `row_index` - Index of this row in the table
/// * `has_span` - Whether table has colspan/rowspan
/// * `rowspan_tracker` - Mutable array tracking rowspan remainder for each column
/// * `total_cols` - Total columns in the table
/// * `header_cols` - Columns to render in separator row
/// * `dom_ctx` - DOM context
/// * `depth` - Nesting depth
/// * `is_header` - Whether this is a header row
/// * `col_widths` - Per-column max content widths for padding (empty = no padding)
/// * `cell_cache` - Markdown already rendered for these cells by the width pre-pass
#[allow(clippy::too_many_arguments)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn convert_table_row(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::super::super::Context,
    row_index: usize,
    has_span: bool,
    rowspan_tracker: &mut [Option<usize>],
    total_cols: usize,
    header_cols: usize,
    dom_ctx: &super::super::super::DomContext,
    depth: usize,
    is_header: bool,
    col_widths: &[usize],
    cell_cache: &mut CellTextCache,
) {
    let mut row_text = String::with_capacity(256);
    let mut cells = Vec::new();

    collect_table_cells(node_handle, parser, dom_ctx, &mut cells);

    #[cfg(feature = "visitor")]
    let cell_contents: Vec<String> = if ctx.visitor.is_some() {
        // ~keep Same rule as the width pre-pass: this walk only feeds the `visit_table_row`
        // ~keep callback, the render pass below walks these cells again (a visitor disables
        // ~keep cell-text reuse), so every shared `Rc` collector is detached to keep exactly
        // ~keep one recording walk per cell.
        let mut collect_ctx = super::super::super::Context {
            in_table_cell: true,
            ..ctx.clone()
        };
        #[cfg(feature = "metadata")]
        {
            collect_ctx.metadata_collector = None;
        }
        collect_ctx.structure_collector = None;
        #[cfg(feature = "inline-images")]
        {
            collect_ctx.inline_collector = None;
        }
        cells
            .iter()
            .map(|cell_handle| {
                let mut text = String::new();
                if let Some(tl::Node::Tag(tag)) = cell_handle.get(parser) {
                    for child_handle in tag.children().top().iter() {
                        super::super::super::walk_node(
                            child_handle,
                            parser,
                            &mut text,
                            options,
                            &collect_ctx,
                            depth + 1,
                            dom_ctx,
                        );
                    }
                }
                crate::text::normalize_whitespace_cow(&text).trim().to_string()
            })
            .collect()
    } else {
        Vec::new()
    };

    #[cfg(feature = "visitor")]
    if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::TableRow,
                Cow::Borrowed("tr"),
                tag,
                depth,
                row_index,
                Some(Cow::Borrowed("table")),
                false,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_table_row(&node_ctx, &cell_contents, is_header)
            };
            match visit_result {
                VisitResult::Continue => {}
                VisitResult::Skip => return,
                VisitResult::Custom(custom) => {
                    output.push_str(&custom);
                    return;
                }
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                    return;
                }
                VisitResult::PreserveHtml => {
                    output.push_str(&super::super::super::serialize_node(node_handle, parser));
                    return;
                }
            }
        }
    }

    // ~keep Build the per-cell context once for the entire row.  Tier-2 hot-spot
    // ~keep pass III: avoids cloning `Context` (which holds several Rc<HashSet> and
    // ~keep optional collector handles) on every cell in wikipedia-class tables.
    let cell_ctx = super::super::super::Context {
        in_table_cell: true,
        ..ctx.clone()
    };

    let mut filled_cols = if has_span {
        let mut col_index = 0;
        let mut cell_iter = cells.iter();

        loop {
            if col_index < total_cols {
                if let Some(Some(remaining_rows)) = rowspan_tracker.get_mut(col_index) {
                    if *remaining_rows > 0 {
                        let width = col_widths.get(col_index).copied();
                        row_text.push(' ');
                        if let Some(w) = width {
                            for _ in 0..w {
                                row_text.push(' ');
                            }
                        }
                        row_text.push_str(" |");
                        *remaining_rows -= 1;
                        if *remaining_rows == 0 {
                            rowspan_tracker[col_index] = None;
                        }
                        col_index += 1;
                        continue;
                    }
                }
            }

            if let Some(cell_handle) = cell_iter.next() {
                let col_width = col_widths.get(col_index).copied();
                emit_row_cell(
                    cell_handle,
                    parser,
                    &mut row_text,
                    options,
                    &cell_ctx,
                    dom_ctx,
                    col_width,
                    depth + 1,
                    cell_cache,
                );

                let (colspan, rowspan) = get_colspan_rowspan(cell_handle, parser);

                if rowspan > 1 && col_index < total_cols {
                    rowspan_tracker[col_index] = Some(rowspan - 1);
                }

                col_index = col_index.saturating_add(colspan);
            } else {
                break;
            }
        }
        col_index
    } else {
        for (cell_idx, cell_handle) in cells.iter().enumerate() {
            let col_width = col_widths.get(cell_idx).copied();
            emit_row_cell(
                cell_handle,
                parser,
                &mut row_text,
                options,
                &cell_ctx,
                dom_ctx,
                col_width,
                depth + 1,
                cell_cache,
            );
        }
        cells.len()
    };

    // ~keep A ragged row with fewer actual cells than the table's widest row must still
    // ~keep declare `total_cols` columns: GFM requires the header row's cell count to match
    // ~keep the delimiter row exactly, or the whole construct fails to parse as a table at
    // ~keep all (issue #13). Padding every row keeps column counts consistent throughout.
    while filled_cols < total_cols {
        let width = col_widths.get(filled_cols).copied();
        row_text.push(' ');
        if let Some(w) = width {
            for _ in 0..w {
                row_text.push(' ');
            }
        }
        row_text.push_str(" |");
        filled_cols += 1;
    }

    output.push('|');
    output.push_str(&row_text);
    output.push('\n');

    let is_first_row = row_index == 0;
    if is_first_row {
        let total_cols = header_cols.clamp(1, MAX_TABLE_COLS);
        output.push_str("| ");
        for i in 0..total_cols {
            if i > 0 {
                output.push_str(" | ");
            }
            let dash_count = col_widths.get(i).copied().unwrap_or(0).max(MIN_SEPARATOR_DASHES);
            for _ in 0..dash_count {
                output.push('-');
            }
        }
        output.push_str(" |\n");
    }
}
