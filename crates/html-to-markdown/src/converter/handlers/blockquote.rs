//! Blockquote element handler for HTML to Markdown conversion.
//!
//! Handles `<blockquote>` elements including:
//! - Basic blockquote markdown output with `> ` prefix
//! - Nested blockquotes
//! - Citation URLs via `cite` attribute
//! - Visitor callback integration

use crate::converter::Context;
use crate::converter::dom_context::DomContext;
use crate::converter::main::walk_node;
use crate::converter::main_helpers::strip_trailing_backslash_breaks_from_fresh_buffer;
use crate::options::ConversionOptions;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node_to_html;
#[cfg(feature = "visitor")]
use std::borrow::Cow;

/// Handle a `<blockquote>` element and convert to Markdown.
///
/// This handler processes blockquote elements including:
/// - Converting inline blockquotes by processing children as inline
/// - Handling nested blockquotes via `blockquote_depth` tracking
/// - Processing citation URLs from cite attribute
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Adding proper spacing and blockquote prefix formatting
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_blockquote(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    if ctx.convert_as_inline {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
        return;
    }

    let cite = tag
        .attributes()
        .get("cite")
        .flatten()
        .map(|v| v.as_utf8_str().to_string());

    let blockquote_ctx = Context {
        blockquote_depth: ctx.blockquote_depth + 1,
        ..ctx.clone()
    };

    let mut content = String::with_capacity(256);
    let children = tag.children();
    {
        for child_handle in children.top().iter() {
            walk_node(
                child_handle,
                parser,
                &mut content,
                options,
                &blockquote_ctx,
                depth + 1,
                dom_ctx,
            );
        }
    }

    // ~keep A trailing <br> run with no following sibling has no next dispatch to catch it
    // ~keep in `walk_node`'s pre-block-dispatch strip, since the blockquote's content is
    // ~keep simply finished here — so this closes its own trailing run the same way
    // ~keep `paragraph.rs` closes its own (issue #464 follow-up).
    strip_trailing_backslash_breaks_from_fresh_buffer(&mut content, options.newline_style);

    let trimmed_content = content.trim();

    #[cfg(feature = "visitor")]
    if let Some(ref visitor) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_lazy_attributes(
            NodeType::Blockquote,
            Cow::Borrowed("blockquote"),
            tag,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            false,
        );

        let mut visitor_ref = visitor.lock().expect("visitor mutex poisoned");
        match visitor_ref.visit_blockquote(&node_ctx, trimmed_content, ctx.blockquote_depth) {
            VisitResult::Continue => {}
            VisitResult::Custom(custom) => {
                output.push_str(&custom);
                return;
            }
            VisitResult::Skip => return,
            VisitResult::PreserveHtml => {
                let mut html_output = String::new();
                serialize_node_to_html(node_handle, parser, &mut html_output);
                output.push_str(&html_output);
                return;
            }
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                return;
            }
        }
    }

    if !trimmed_content.is_empty() {
        // ~keep Only the outermost blockquote call writes into the real document buffer —
        // ~keep a nested blockquote's own call writes into its parent's local `content`
        // ~keep scratch buffer instead (see above), which the parent then re-prefixes with
        // ~keep its own "> " on the way out. Applying the list continuation indent at every
        // ~keep nesting level would stack it once per level; restricting it to
        // ~keep `blockquote_depth == 0` applies it exactly once, at the boundary where this
        // ~keep content actually reaches the list item's own text.
        let list_indent = if ctx.in_list_item && ctx.blockquote_depth == 0 {
            crate::converter::list::utils::continuation_indent_string(ctx.list_depth, ctx.list_indent_columns, options)
        } else {
            None
        };

        // ~keep A blockquote that continues already-started list item content needs its
        // ~keep first quoted line indented too; one that is the item's first content
        // ~keep instead sits right after the marker, which already provides that column
        // ~keep (see `block/paragraph.rs::add_list_continuation_indent` for the identical
        // ~keep first-line distinction, applied there to paragraphs only).
        let is_list_continuation = list_indent.is_some()
            && !output.is_empty()
            && !output.ends_with("* ")
            && !output.ends_with("- ")
            && !output.ends_with(". ");

        if ctx.blockquote_depth > 0 {
            if !output.is_empty() {
                while output.ends_with('\n') {
                    output.truncate(output.len() - 1);
                }
                output.push_str("\n\n");
            }
        } else if !output.is_empty() {
            if output.ends_with("\n\n") {
                output.truncate(output.len() - 1);
            } else if !output.ends_with('\n') {
                output.push_str("\n\n");
            } else if !output.ends_with("\n\n") {
                output.push('\n');
            }
        }

        let prefix = "> ";

        // ~keep Only blank-out whitespace-only lines; preserve leading whitespace on
        // ~keep real content lines (code block indentation, nested list markers) so
        // ~keep quoted block children keep their structural meaning (issue #13).
        //
        // ~keep Every physical line also needs the list item's own continuation indent
        // ~keep when this blockquote is inside a list item — CommonMark's list container
        // ~keep match is per physical line, so an unindented "> " line drops the rest of
        // ~keep the quote (and the item) out of the list on re-parse (spec example 263).
        for (index, line) in trimmed_content.lines().enumerate() {
            if let Some(ref indent) = list_indent {
                if index > 0 || is_list_continuation {
                    output.push_str(indent);
                }
            }
            output.push_str(prefix);
            if !line.trim().is_empty() {
                output.push_str(line);
            }
            output.push('\n');
        }

        if let Some(url) = cite {
            output.push('\n');
            if let Some(ref indent) = list_indent {
                output.push_str(indent);
            }
            output.push_str("— <");
            output.push_str(&url);
            output.push_str(">\n\n");
        }

        // ~keep Add trailing newlines only when appropriate for proper spacing
        // ~keep (matching paragraph conditional logic for CommonMark compliance)
        if !ctx.convert_as_inline && !ctx.in_table_cell && !ctx.in_list_item {
            while output.ends_with('\n') {
                output.truncate(output.len() - 1);
            }
            output.push_str("\n\n");
        }
    }
}
