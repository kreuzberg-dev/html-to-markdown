//! Link element handler for HTML to Markdown conversion.
//!
//! Handles `<a>` elements including:
//! - Basic link markdown output `[text](href "title")`
//! - Autolinks when text matches href
//! - Links containing heading elements
//! - Complex link content with mixed block/inline elements
//! - Visitor callback integration
//! - Link metadata collection

#[cfg(feature = "metadata")]
use std::collections::BTreeMap;

use crate::converter::Context;
use crate::converter::block::heading::{find_single_heading_child, heading_allows_inline_images, push_heading};
use crate::converter::dom_context::DomContext;
use crate::converter::inline::link::{append_markdown_link, has_uri_scheme};
use crate::converter::main::walk_node;
use crate::converter::utility::content::{
    collect_link_label_text, escape_link_label, get_text_content, normalize_link_label, normalized_tag_name,
};
use crate::options::ConversionOptions;
use crate::text;
use std::borrow::Cow;

#[cfg(feature = "visitor")]
use crate::converter::utility::serialization::serialize_node;

/// Handle an `<a>` (link) element and convert to Markdown.
///
/// This handler processes link elements including:
/// - Extracting href and title attributes
/// - Detecting autolinks (where text equals href)
/// - Handling links that contain heading elements
/// - Processing complex link content (mixed block/inline)
/// - Invoking visitor callbacks when the visitor feature is enabled
/// - Collecting link metadata when the metadata feature is enabled
/// - Generating appropriate markdown link output
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
#[cfg_attr(not(feature = "visitor"), allow(unused_variables))]
pub fn handle_link(
    node_handle: &tl::NodeHandle,
    tag: &tl::HTMLTag,
    parser: &tl::Parser,
    output: &mut String,
    options: &ConversionOptions,
    ctx: &Context,
    depth: usize,
    dom_ctx: &DomContext,
) {
    let href_attr = tag
        .attributes()
        .get("href")
        .flatten()
        .map(|v| text::decode_html_entities(&v.as_utf8_str()));
    // ~keep An empty `title=""` carries no information, and `[t](u "")` / `![a](i "")` is
    // ~keep noise that no Markdown serializer round-trips: re-rendering the output drops
    // ~keep the empty title, so the second pass no longer matches the first. Treat it as
    // ~keep absent, which is what it means.
    let title = tag
        .attributes()
        .get("title")
        .flatten()
        .map(|v| v.as_utf8_str())
        .filter(|v| !v.is_empty());

    if let Some(href) = href_attr {
        let owned_children: Vec<tl::NodeHandle>;
        let children: &[tl::NodeHandle] = if let Some(c) = dom_ctx.children_of(node_handle.get_inner()) {
            c.as_slice()
        } else {
            owned_children = tag.children().top().iter().copied().collect();
            owned_children.as_slice()
        };
        let (inline_label, _block_nodes, saw_block) = collect_link_label_text(children, parser, dom_ctx);

        // ~keep Without block descendants the sweep above already visited exactly the nodes
        // ~keep `get_text_content` would and decoded them the same way, so its text is reused
        // ~keep rather than walking the `<a>` subtree a second time.
        let text_source: Cow<'_, str> = if saw_block {
            Cow::Owned(get_text_content(node_handle, parser, dom_ctx))
        } else {
            Cow::Borrowed(inline_label.as_str())
        };
        let normalized_text = text::normalize_whitespace_cow(text_source.as_ref());
        let raw_text = normalized_text.trim();

        // ~keep GFM requires an absolute URI with a scheme (e.g. `https://…`, `mailto:…`);
        // ~keep bare paths or filenames must use the full `[text](href)` form (issue #397).
        let is_autolink = options.autolinks
            && !options.default_title
            && !href.is_empty()
            && has_uri_scheme(href.as_str())
            && (raw_text == href || (href.starts_with("mailto:") && raw_text == &href[7..]));

        if is_autolink {
            output.push('<');
            if href.starts_with("mailto:") && raw_text == &href[7..] {
                output.push_str(raw_text);
            } else {
                output.push_str(&href);
            }
            output.push('>');
            return;
        }

        if let Some((heading_level, heading_handle)) = find_single_heading_child(*node_handle, parser) {
            if let Some(heading_node) = heading_handle.get(parser) {
                if let tl::Node::Tag(heading_tag) = heading_node {
                    let heading_name = normalized_tag_name(heading_tag.name().as_utf8_str()).into_owned();
                    let mut heading_text = String::new();
                    let heading_ctx = Context {
                        in_heading: true,
                        convert_as_inline: true,
                        heading_allow_inline_images: heading_allows_inline_images(
                            &heading_name,
                            &ctx.keep_inline_images_in,
                        ),
                        ..ctx.clone()
                    };
                    walk_node(
                        &heading_handle,
                        parser,
                        &mut heading_text,
                        options,
                        &heading_ctx,
                        depth + 1,
                        dom_ctx,
                    );
                    let trimmed_heading = heading_text.trim();
                    if !trimmed_heading.is_empty() {
                        let escaped_label = escape_link_label(trimmed_heading);
                        let mut link_buffer = String::new();
                        append_markdown_link(
                            &mut link_buffer,
                            &escaped_label,
                            href.as_str(),
                            title.as_deref(),
                            raw_text,
                            options,
                            ctx.reference_collector.as_ref(),
                        );
                        push_heading(output, ctx, options, heading_level, link_buffer.as_str());
                        return;
                    }
                }
            }
        }

        let mut label = if saw_block {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                convert_as_inline: true,
                ..ctx.clone()
            };
            for child_handle in children {
                let mut child_buf = String::new();
                walk_node(
                    child_handle,
                    parser,
                    &mut child_buf,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
                if !child_buf.trim().is_empty()
                    && !content.is_empty()
                    && !content.chars().last().is_none_or(char::is_whitespace)
                    && !child_buf.chars().next().is_none_or(char::is_whitespace)
                {
                    content.push(' ');
                }
                content.push_str(&child_buf);
            }
            if content.trim().is_empty() {
                normalize_link_label(&inline_label)
            } else {
                normalize_link_label(&content)
            }
        } else {
            let mut content = String::new();
            let link_ctx = Context {
                inline_depth: ctx.inline_depth + 1,
                ..ctx.clone()
            };
            for child_handle in children {
                walk_node(
                    child_handle,
                    parser,
                    &mut content,
                    options,
                    &link_ctx,
                    depth + 1,
                    dom_ctx,
                );
            }
            normalize_link_label(&content)
        };

        // ~keep `raw_text` is already the whole-subtree text when `saw_block`, so this single
        // ~keep fallback covers both the block and inline cases.
        if label.is_empty() && !raw_text.is_empty() {
            label = normalize_link_label(raw_text);
        }

        if label.is_empty() && !href.is_empty() && !children.is_empty() {
            label.clone_from(&href);
        }

        if label == "^" && href.starts_with('#') {
            label = "↑".to_string();
        }

        let escaped_label = escape_link_label(&label);

        #[cfg(feature = "visitor")]
        if let Some(ref visitor_handle) = ctx.visitor {
            use crate::visitor::{NodeContext, NodeType, VisitResult};

            let node_id = node_handle.get_inner();
            let parent_tag = dom_ctx.parent_tag_name(node_id, parser);
            let index_in_parent = dom_ctx.get_sibling_index(node_id).unwrap_or(0);

            let node_ctx = NodeContext::with_lazy_attributes(
                NodeType::Link,
                Cow::Borrowed("a"),
                tag,
                depth,
                index_in_parent,
                parent_tag.map(Cow::Borrowed),
                true,
            );

            let visit_result = {
                let mut visitor = visitor_handle.lock().expect("visitor mutex poisoned");
                visitor.visit_link(&node_ctx, &href, &label, title.as_deref())
            };
            match visit_result {
                VisitResult::Continue => append_markdown_link(
                    output,
                    &escaped_label,
                    href.as_str(),
                    title.as_deref(),
                    label.as_str(),
                    options,
                    ctx.reference_collector.as_ref(),
                ),
                VisitResult::Custom(custom) => output.push_str(&custom),
                VisitResult::Skip => {}
                VisitResult::Error(err) => {
                    if ctx.visitor_error.borrow().is_none() {
                        *ctx.visitor_error.borrow_mut() = Some(err);
                    }
                }
                VisitResult::PreserveHtml => output.push_str(&serialize_node(node_handle, parser)),
            }
        } else {
            append_markdown_link(
                output,
                &escaped_label,
                href.as_str(),
                title.as_deref(),
                label.as_str(),
                options,
                ctx.reference_collector.as_ref(),
            );
        }

        #[cfg(not(feature = "visitor"))]
        append_markdown_link(
            output,
            &escaped_label,
            href.as_str(),
            title.as_deref(),
            label.as_str(),
            options,
            ctx.reference_collector.as_ref(),
        );

        #[cfg(feature = "metadata")]
        if ctx.metadata_wants_links {
            if let Some(ref collector) = ctx.metadata_collector {
                let rel_attr = tag
                    .attributes()
                    .get("rel")
                    .flatten()
                    .map(|v| v.as_utf8_str().to_string());
                let mut attributes_map = BTreeMap::new();
                for (key, value_opt) in tag.attributes().iter() {
                    let key_str = key.to_string();
                    if key_str == "href" {
                        continue;
                    }

                    let value = value_opt.map(|v| v.to_string()).unwrap_or_default();
                    attributes_map.insert(key_str, value);
                }
                collector.borrow_mut().add_link(
                    href.clone(),
                    label,
                    title.as_deref().map(str::to_string),
                    rel_attr,
                    attributes_map,
                );
            }
        }
    } else {
        let children = tag.children();
        {
            for child_handle in children.top().iter() {
                walk_node(child_handle, parser, output, options, ctx, depth + 1, dom_ctx);
            }
        }
    }
}
