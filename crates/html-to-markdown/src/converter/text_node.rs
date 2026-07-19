//! Text node processing for HTML to Markdown conversion.
//!
//! Handles raw text nodes with:
//! - HTML entity decoding
//! - Whitespace normalization and stripping
//! - Text escaping with configurable escape modes
//! - Visitor callbacks (when feature enabled)
//! - List item indentation

use std::borrow::Cow;

use crate::converter::dom_context::DomContext;
use crate::converter::main_helpers::{has_more_than_one_char, is_inline_element};
use crate::converter::utility::siblings::{
    get_next_sibling_tag, next_sibling_is_inline_tag, previous_sibling_is_inline_tag,
};
use crate::options::ConversionOptions;
use crate::text;
#[cfg(feature = "visitor")]
use crate::visitor::EMPTY_ATTRS;

type Context = crate::converter::Context;

/// Process a raw text node during HTML to Markdown conversion.
///
/// Handles:
/// - HTML entity decoding
/// - Whitespace normalization and stripping
/// - Text escaping with configurable escape modes
/// - Visitor callbacks (when feature enabled)
/// - List item indentation
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn process_text_node(
    raw: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let mut text = text::decode_html_entities_cow(raw);

    if text.is_empty() {
        return;
    }

    let text_ref = text.as_ref();
    let had_newlines = text_ref.contains('\n');
    let has_double_newline = text_ref.contains("\n\n") || text_ref.contains("\r\n\r\n");

    if options.strip_newlines && (text.contains('\r') || text.contains('\n')) {
        text = Cow::Owned(text.replace(['\r', '\n'], " "));
    }

    if text.trim().is_empty() {
        if ctx.in_code {
            output.push_str(text.as_ref());
            return;
        }

        if options.whitespace_mode == crate::options::WhitespaceMode::Strict {
            if ctx.convert_as_inline || ctx.in_table_cell || ctx.in_list_item {
                output.push_str(text.as_ref());
                return;
            }
            if has_double_newline {
                if !output.ends_with("\n\n") {
                    output.push('\n');
                }
                return;
            }
            output.push_str(text.as_ref());
            return;
        }

        if had_newlines {
            if output.is_empty() {
                return;
            }
            if !output.ends_with("\n\n") {
                if let Some(next_tag) = get_next_sibling_tag(node_handle, parser, dom_ctx) {
                    if is_inline_element(next_tag) {
                        if !output.ends_with(' ') && !output.ends_with('\n') {
                            output.push(' ');
                        }
                        return;
                    }
                } else if newline_span_needs_separating_space(node_handle, parser, dom_ctx)
                    && !output.ends_with(' ')
                    && !output.ends_with('\n')
                {
                    // ~keep issue #430: a lone "\n" inside an inline wrapper (e.g. a
                    // ~keep <span>) has no in-parent next sibling, but when that wrapper
                    // ~keep is itself followed by inline content the newline still
                    // ~keep separates words — collapse it to a single space.
                    output.push(' ');
                    return;
                }
            }
            return;
        }

        if previous_sibling_is_inline_tag(node_handle, parser, dom_ctx)
            && next_sibling_is_inline_tag(node_handle, parser, dom_ctx)
        {
            if has_more_than_one_char(text.as_ref()) {
                if !output.ends_with(' ') {
                    output.push(' ');
                }
            } else {
                output.push_str(text.as_ref());
            }
        } else {
            output.push_str(text.as_ref());
        }
        return;
    }

    let processed_text = if ctx.in_code || ctx.in_ruby {
        text.into_owned()
    } else if ctx.in_table_cell {
        // ~keep Always escape * and _ in table cells to prevent unintended emphasis.
        // ~keep When escape_misc is false the previous implementation appended a
        // ~keep post-pass `String::replace('|', "\\|")`.  We fold the pipe escape
        // ~keep into the misc set so the byte-loop handles it in the same walk,
        // ~keep avoiding a second allocation.
        let normalized_text = if options.whitespace_mode == crate::options::WhitespaceMode::Normalized {
            text::normalize_whitespace_cow(text.as_ref())
        } else {
            Cow::Borrowed(text.as_ref())
        };
        let src = normalized_text.as_ref();
        let mut out = String::with_capacity(src.len());
        text::escape_into(&mut out, src, options.escape_misc, true, true, options.escape_ascii);
        if !options.escape_misc {
            if out.contains('|') {
                out = out.replace('|', r"\|");
            }
        }
        out
    } else if options.whitespace_mode == crate::options::WhitespaceMode::Strict {
        text::escape(
            text.as_ref(),
            options.escape_misc,
            options.escape_asterisks,
            options.escape_underscores,
            options.escape_ascii,
        )
        .into_owned()
    } else {
        let has_double_newline = text.contains("\n\n") || text.contains("\r\n\r\n");
        let has_trailing_single_newline =
            text.ends_with('\n') && !text.ends_with("\n\n") && !text.ends_with("\r\n\r\n");

        let normalized_text = text::normalize_whitespace_cow(text.as_ref());

        let (prefix, suffix, core) = text::chomp(normalized_text.as_ref());

        let skip_prefix = output.ends_with("\n\n")
            || output.ends_with("* ")
            || output.ends_with("- ")
            || output.ends_with(". ")
            || output.ends_with("] ")
            || (output.ends_with('\n') && prefix == " ")
            || (output.ends_with(' ')
                && prefix == " "
                && !previous_sibling_is_inline_tag(node_handle, parser, dom_ctx));

        let mut final_text = String::with_capacity(prefix.len() + core.len() + suffix.len() + 2);
        if !skip_prefix && !prefix.is_empty() {
            final_text.push_str(prefix);
        }

        let escaped_core = text::escape(
            core,
            options.escape_misc,
            options.escape_asterisks,
            options.escape_underscores,
            options.escape_ascii,
        );
        final_text.push_str(&escaped_core);

        if !suffix.is_empty() {
            final_text.push_str(suffix);
        } else if has_trailing_single_newline {
            let safe_start = ctx.block_content_start.min(output.len());
            let safe_start = crate::converter::utility::content::floor_char_boundary(output, safe_start);
            let current_block_output = &output[safe_start..];
            let at_paragraph_break = current_block_output.ends_with("\n\n");
            if !at_paragraph_break {
                if has_double_newline {
                    final_text.push('\n');
                } else if let Some(next_tag) = get_next_sibling_tag(node_handle, parser, dom_ctx) {
                    if matches!(next_tag, "span") {
                    } else if ctx.inline_depth > 0 || ctx.convert_as_inline || ctx.in_paragraph {
                        final_text.push(' ');
                    } else {
                        final_text.push('\n');
                    }
                } else if ctx.inline_depth > 0 || ctx.convert_as_inline || ctx.in_paragraph {
                    final_text.push(' ');
                } else {
                    final_text.push('\n');
                }
            }
        }

        final_text
    };

    #[cfg(feature = "visitor")]
    let final_text = if let Some(ref visitor_handle) = ctx.visitor {
        use crate::visitor::{NodeContext, NodeType, VisitResult};

        let node_id = node_handle.get_inner();
        let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
        let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

        let node_ctx = NodeContext::with_borrowed_attributes(
            NodeType::Text,
            Cow::Borrowed(""),
            &EMPTY_ATTRS,
            depth,
            index_in_parent,
            parent_tag.map(Cow::Borrowed),
            true,
        );

        let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
        match visitor.visit_text(&node_ctx, &processed_text) {
            VisitResult::Continue => processed_text,
            VisitResult::Custom(custom) => {
                if ctx.inline_depth > 0 || ctx.in_heading {
                    processed_text
                } else {
                    custom
                }
            }
            VisitResult::Skip => return,
            VisitResult::Error(err) => {
                if ctx.visitor_error.borrow().is_none() {
                    *ctx.visitor_error.borrow_mut() = Some(err);
                }
                return;
            }
            VisitResult::PreserveHtml => processed_text,
        }
    } else {
        processed_text
    };

    #[cfg(not(feature = "visitor"))]
    let final_text = processed_text;

    if ctx.in_list_item && final_text.contains("\n\n") {
        let indent = " ".repeat(4 * ctx.list_depth);
        let mut first = true;
        for part in final_text.split("\n\n") {
            if !first {
                output.push_str("\n\n");
                output.push_str(&indent);
            }
            first = false;
            output.push_str(part.trim());
        }
    } else {
        output.push_str(&final_text);
    }
}

/// Whether a whitespace-only newline text node with no in-parent next sibling
/// still separates inline content (issue #430).
///
/// Returns true when the node's parent is an inline-like element that is itself
/// followed by inline content — e.g. the lone `"\n"` inside the middle `<span>`
/// of `<span>a</span><span>\n</span><span>b</span>`.
fn newline_span_needs_separating_space(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> bool {
    let Some(parent_id) = dom_ctx.parent_of(node_handle.get_inner()) else {
        return false;
    };
    let parent_is_inline = dom_ctx
        .tag_info(parent_id, parser)
        .is_some_and(|info| info.is_inline_like);
    if !parent_is_inline {
        return false;
    }
    let parent_handle = tl::NodeHandle::new(parent_id);
    next_sibling_is_inline_tag(&parent_handle, parser, dom_ctx)
}
