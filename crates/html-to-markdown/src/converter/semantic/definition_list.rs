//! Handlers for HTML5 definition list and heading group elements.
//!
//! Processes list and heading semantic elements:
//! - `<hgroup>` - Groups related headings together
//! - `<dl>` - Definition list container
//! - `<dt>` - Definition term
//! - `<dd>` - Definition description
//! - `<menu>` - Semantic list (typically unordered)
//!
//! These elements have special formatting requirements for proper Markdown output.

use super::walk_node;

/// Handles the `<hgroup>` element.
///
/// An hgroup element groups related headings together (e.g., a title and subtitle).
/// In Markdown, we simply process all children sequentially, allowing nested
/// headings to maintain their individual formatting.
///
/// # Behavior
///
/// - Children are processed sequentially in the current context
/// - No special formatting is applied at the hgroup level
pub fn handle_hgroup(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
    }
}

/// Handles the `<dl>` element.
///
/// A definition list contains terms and their definitions. Terms and definitions
/// are output as plain blocks without Pandoc-style colon syntax, since standard
/// Markdown and GFM do not support definition lists.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline without block spacing
/// - **Block mode**: Content is collected and wrapped with proper spacing
pub fn handle_dl(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        if ctx.convert_as_inline {
            let children = tag.children();
            {
                for child_handle in children.top().iter() {
                    walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
                }
            }
            return;
        }

        let mut content = String::new();
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        let trimmed = content.trim();
        if !trimmed.is_empty() {
            if !output.is_empty() && !output.ends_with("\n\n") {
                output.push_str("\n\n");
            }
            output.push_str(trimmed);
            output.push_str("\n\n");
        }
    }
}

/// Handles the `<dt>` element.
///
/// A dt element contains a term being defined. Terms are output on their own line,
/// with definitions following on subsequent lines.
///
/// # Behavior
///
/// - **Inline mode**: Content is output as-is
/// - **Block mode**: Content is followed by a newline
pub fn handle_dt(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let mut content = String::with_capacity(64);
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        let trimmed = content.trim();
        if !trimmed.is_empty() {
            if ctx.convert_as_inline {
                output.push_str(trimmed);
            } else {
                output.push_str(trimmed);
                output.push('\n');
            }
        }
    }
}

/// Handles the `<dd>` element.
///
/// A dd element contains the definition for a term. It is output as a plain
/// block since standard Markdown and GFM do not support definition list syntax.
///
/// # Behavior
///
/// - **Inline mode**: Content is output as-is
/// - **Block mode**: Content is output as a block
pub fn handle_dd(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let mut content = String::with_capacity(128);
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, &mut content, options, ctx, depth + 1, dom_ctx);
            }
        }

        let trimmed = content.trim();

        if ctx.convert_as_inline {
            if !trimmed.is_empty() {
                output.push_str(trimmed);
            }
        } else if !trimmed.is_empty() {
            output.push_str(trimmed);
            output.push_str("\n\n");
        }
    }
}

/// Handles the `<menu>` element.
///
/// A menu element is a semantic list, typically used for command menus or
/// navigation. It is rendered as an unordered list with dashes.
///
/// # Behavior
///
/// - **Inline mode**: Children are processed inline without list formatting
/// - **Block mode**: Content is rendered as an unordered list
/// - Uses `-` as the list bullet (overrides configured bullets)
/// - Proper blank-line spacing is maintained
pub fn handle_menu(
    _tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    if let Some(tl::Node::Tag(tag)) = node_handle.get(parser) {
        let content_start = output.len();

        let menu_options = crate::options::ConversionOptions {
            bullets: "-".to_string(),
            ..options.clone()
        };

        let list_ctx = super::Context {
            in_ordered_list: false,
            list_counter: 0,
            in_list: true,
            list_depth: ctx.list_depth,
            ..ctx.clone()
        };

        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(
                    child_handle,
                    parser,
                    output,
                    &menu_options,
                    &list_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
        }

        if !ctx.convert_as_inline && output.len() > content_start {
            if !output.ends_with("\n\n") {
                if output.ends_with('\n') {
                    output.push('\n');
                } else {
                    output.push_str("\n\n");
                }
            }
        } else if ctx.convert_as_inline {
            while output.ends_with('\n') {
                output.pop();
            }
        }
    }
}

/// Dispatcher for definition list and related elements.
///
/// Routes `<hgroup>`, `<dl>`, `<dt>`, `<dd>`, and `<menu>` elements
/// to their respective handlers.
pub fn handle(
    tag_name: &str,
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    output: &mut String,
    options: &crate::options::ConversionOptions,
    ctx: &super::Context,
    depth: usize,
    dom_ctx: &super::DomContext,
) {
    match tag_name {
        "hgroup" => handle_hgroup(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dl" => handle_dl(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dt" => handle_dt(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "dd" => handle_dd(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        "menu" => handle_menu(tag_name, node_handle, parser, output, options, ctx, depth, dom_ctx),
        _ => {}
    }
}
