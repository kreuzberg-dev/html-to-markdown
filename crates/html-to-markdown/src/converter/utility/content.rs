//! Content extraction and manipulation utilities.
//!
//! Functions for extracting and processing element content, including text collection
//! and empty element detection.

use crate::text;
use std::borrow::Cow;
#[cfg(feature = "visitor")]
use std::collections::BTreeMap;

pub use crate::converter::DomContext;

/// Collect all attributes from an HTML tag as a `BTreeMap<String, String>`.
///
/// Boolean attributes (those with `None` as the value) are skipped; only
/// attributes that carry an explicit value are included.
#[cfg(feature = "visitor")]
pub fn collect_tag_attributes(tag: &tl::HTMLTag) -> BTreeMap<String, String> {
    tag.attributes()
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| (k.to_string(), val.to_string())))
        .collect()
}

/// Chomp whitespace from inline element content, preserving line breaks.
///
/// Similar to `text::chomp` but handles line breaks from `<br>` tags specially.
/// Line breaks are extracted as suffix to be placed outside formatting.
/// Returns (prefix, suffix, `trimmed_text`).
pub fn chomp_inline(text: &str) -> (&str, &str, &str) {
    if text.is_empty() {
        return ("", "", "");
    }

    let prefix = if text.starts_with(&[' ', '\t'][..]) { " " } else { "" };

    let has_trailing_linebreak = text.ends_with("  \n") || text.ends_with("\\\n");

    let suffix = if has_trailing_linebreak {
        if text.ends_with("  \n") { "  \n" } else { "\\\n" }
    } else if text.ends_with(&[' ', '\t'][..]) {
        " "
    } else {
        ""
    };

    let trimmed = if has_trailing_linebreak {
        text.strip_suffix("  \n").map_or_else(
            || text.strip_suffix("\\\n").map_or_else(|| text.trim(), |s| s.trim()),
            |s| s.trim(),
        )
    } else {
        text.trim()
    };

    (prefix, suffix, trimmed)
}

/// Get the text content of a node and its children.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn get_text_content(node_handle: &tl::NodeHandle, parser: &tl::Parser, dom_ctx: &DomContext) -> String {
    dom_ctx.text_content(*node_handle, parser)
}

/// Collect inline text for link labels, skipping block-level descendants.
#[allow(clippy::match_wildcard_for_single_variants)]
pub fn collect_link_label_text(
    children: &[tl::NodeHandle],
    parser: &tl::Parser,
    dom_ctx: &DomContext,
) -> (String, Vec<tl::NodeHandle>, bool) {
    let mut text = String::new();
    let mut saw_block = false;
    let mut block_nodes = Vec::new();
    let mut stack: Vec<_> = children.iter().rev().copied().collect();

    while let Some(handle) = stack.pop() {
        if let Some(node) = handle.get(parser) {
            match node {
                tl::Node::Raw(bytes) => {
                    let raw = bytes.as_utf8_str();
                    let decoded = text::decode_html_entities_cow(raw.as_ref());
                    text.push_str(decoded.as_ref());
                }
                tl::Node::Tag(tag) => {
                    let is_block = dom_ctx.tag_info(handle.get_inner(), parser).map_or_else(
                        || {
                            let tag_name = normalized_tag_name(tag.name().as_utf8_str());
                            is_block_level_element(tag_name.as_ref())
                        },
                        |info| info.is_block,
                    );
                    if is_block {
                        saw_block = true;
                        block_nodes.push(handle);
                        continue;
                    }

                    if let Some(children) = dom_ctx.children_of(handle.get_inner()) {
                        for child in children.iter().rev() {
                            stack.push(*child);
                        }
                    } else {
                        let tag_children = tag.children();
                        let mut child_nodes: Vec<_> = tag_children.top().iter().copied().collect();
                        child_nodes.reverse();
                        stack.extend(child_nodes);
                    }
                }
                _ => {}
            }
        }
    }

    (text, block_nodes, saw_block)
}

/// Normalize a link label by collapsing newlines and normalizing whitespace.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn normalize_link_label(label: &str) -> String {
    let mut needs_collapse = false;
    for ch in label.chars() {
        if ch == '\n' || ch == '\r' {
            needs_collapse = true;
            break;
        }
    }

    let collapsed = if needs_collapse {
        let mut collapsed = String::with_capacity(label.len());
        for ch in label.chars() {
            if ch == '\n' || ch == '\r' {
                collapsed.push(' ');
            } else {
                collapsed.push(ch);
            }
        }
        Cow::Owned(collapsed)
    } else {
        Cow::Borrowed(label)
    };

    let normalized = text::normalize_whitespace_cow(collapsed.as_ref());
    normalized.as_ref().trim().to_string()
}

/// Normalize a tag name to lowercase, preserving borrowed input when possible.
pub fn normalized_tag_name(raw: Cow<'_, str>) -> Cow<'_, str> {
    if raw.as_bytes().iter().any(u8::is_ascii_uppercase) {
        let mut owned = raw.into_owned();
        owned.make_ascii_lowercase();
        Cow::Owned(owned)
    } else {
        raw
    }
}

/// Check if an element is block-level (not inline).
pub fn is_block_level_element(tag_name: &str) -> bool {
    is_block_level_name(tag_name, crate::converter::main_helpers::is_inline_element(tag_name))
}

/// Returns the largest valid char boundary index at or before `index`.
///
/// If `index` is already a char boundary it is returned unchanged.
/// Otherwise it walks backwards to find one.  Returns 0 if no boundary
/// is found before `index`.
pub const fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let mut i = index;
        while i > 0 && !s.is_char_boundary(i) {
            i -= 1;
        }
        i
    }
}

/// Escape special Markdown characters in a link label or image alt text.
///
/// Handles bracket escaping to prevent unintended link label termination.
/// Tracks matched bracket pairs and escapes a closing bracket that has no local opener
/// (it would otherwise close the caller's own wrapping `[`/`![` early), and escapes a
/// matched pair outright when it is itself link- or reference-link-shaped.
///
/// # Examples
/// ```text
/// Input:  "]"
/// Output: "\\]"
///
/// Input:  "[outer [inner]]"
/// Output: "[outer [inner]]"
///
/// Input:  "[foo](uri2)"
/// Output: "\\[foo\\](uri2)"
/// ```
pub fn escape_link_label(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // ~keep Two linear passes rather than one pass that mutates the output string as it
    // ~keep goes: escaping a matched bracket pair needs to touch the *opener*, which by
    // ~keep the time its `]` is found has already been written. Retroactively
    // ~keep `String::insert`-ing it back in is O(remaining length) per escape, so a label
    // ~keep built of many link-shaped pairs (`[a](b)[a](b)...`) would make the whole
    // ~keep function O(n^2) -- the same denial-of-service shape `bare_lt_complexity.rs`
    // ~keep exists to catch elsewhere in this crate. Precomputing which byte offsets need
    // ~keep an escape first keeps the second pass a single linear append.
    let mut escape_at = vec![false; text.len()];
    // ~keep Byte offsets (into `text`) of unescaped `[` openers not yet matched by a `]`,
    // ~keep innermost last. A bare depth counter cannot tell "is there a local opener"
    // ~keep from "which one", and the `](`/`][` check below needs the specific opener.
    let mut open_positions: Vec<usize> = Vec::new();
    let mut backslash_count = 0usize;
    let mut chars = text.char_indices().peekable();

    while let Some((byte_pos, ch)) = chars.next() {
        if ch == '\\' {
            backslash_count += 1;
            continue;
        }

        let is_escaped = backslash_count % 2 == 1;
        backslash_count = 0;

        match ch {
            '[' if !is_escaped => open_positions.push(byte_pos),
            ']' if !is_escaped => match open_positions.pop() {
                None => {
                    // ~keep No local opener: left alone, this `]` closes the caller's own
                    // ~keep wrapping `[`/`![` early, truncating the label.
                    escape_at[byte_pos] = true;
                }
                Some(open_pos) => {
                    // ~keep A `]` immediately followed by `(` or `[` completes an inline
                    // ~keep link/image (`](dest)`) or a reference link (`][ref]`) on
                    // ~keep reparse: `CommonMark` parses link/image label content as full
                    // ~keep inline markdown, so a literal `[foo](uri2)` inside it silently
                    // ~keep becomes a real nested link and the destination is lost --
                    // ~keep escaping only this closer is not enough and is actively worse:
                    // ~keep the still-open outer `[`/`![` is then free to be captured by a
                    // ~keep *later* unescaped `]` instead (verified against comrak), so the
                    // ~keep image/link fails to form at all. Escaping both ends of the
                    // ~keep matched pair together is the only shape that round-trips.
                    //
                    // ~keep Exempted when the opener is itself preceded by `!`: that is a
                    // ~keep real, intentional `![alt](src)` this converter already emitted
                    // ~keep while walking a nested `<img>` inside link text (CommonMark
                    // ~keep permits images, just not links, inside link text), not literal
                    // ~keep text that merely looks link-shaped -- escaping it would corrupt
                    // ~keep the nested image instead of protecting the label (caught by
                    // ~keep `test_commonmark_compliance`'s `[![moon](moon.jpg)](/uri)`).
                    let is_link_shaped = matches!(chars.peek(), Some((_, '(' | '[')));
                    let is_nested_image = open_pos > 0 && text.as_bytes()[open_pos - 1] == b'!';
                    if is_link_shaped && !is_nested_image {
                        escape_at[open_pos] = true;
                        escape_at[byte_pos] = true;
                    }
                }
            },
            _ => {}
        }
    }

    let mut result = String::with_capacity(text.len() + 2);
    for (byte_pos, ch) in text.char_indices() {
        if escape_at[byte_pos] {
            result.push('\\');
        }
        result.push(ch);
    }

    result
}

/// Helper for block-level element detection.
pub fn is_block_level_name(tag_name: &str, is_inline: bool) -> bool {
    !is_inline
        && matches!(
            tag_name,
            "address"
                | "article"
                | "aside"
                | "blockquote"
                | "canvas"
                | "dd"
                | "div"
                | "dl"
                | "dt"
                | "fieldset"
                | "figcaption"
                | "figure"
                | "footer"
                | "form"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "header"
                | "hr"
                | "li"
                | "main"
                | "nav"
                | "ol"
                | "p"
                | "pre"
                | "section"
                | "table"
                | "tfoot"
                | "ul"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_link_label_leaves_plain_text_unchanged() {
        assert_eq!(escape_link_label("plain text"), "plain text");
    }

    #[test]
    fn escape_link_label_escapes_an_unmatched_closing_bracket() {
        assert_eq!(escape_link_label("]"), "\\]");
    }

    // ~keep A `[...]` pair with a local opener and no trailing `(`/`[` is left unescaped
    // ~keep even though it superficially resembles the unmatched-bracket case above: the
    // ~keep opener/closer here match each other, so unlike a bare `]` they do not close
    // ~keep the caller's own wrapping bracket early.
    #[test]
    fn escape_link_label_leaves_a_matched_non_link_shaped_pair_unchanged() {
        assert_eq!(escape_link_label("[link]"), "[link]");
    }

    #[test]
    fn escape_link_label_leaves_balanced_nested_brackets_unchanged() {
        assert_eq!(escape_link_label("[outer [inner]]"), "[outer [inner]]");
    }

    // ~keep Regression for Cluster B (image alt text losing its nested destination):
    // ~keep `<img alt="[foo](uri2)">` must not let the alt text's own `[foo](uri2)` be
    // ~keep reparsed as a real nested link, or `uri2` is silently dropped on a second
    // ~keep conversion pass (CommonMark parses an image's alt as full inline content).
    #[test]
    fn escape_link_label_escapes_a_link_shaped_bracket_pair() {
        assert_eq!(escape_link_label("[foo](uri2)"), "\\[foo\\](uri2)");
    }

    // ~keep Same hazard, reference-link form: `[foo][ref]` is just as reparsable as
    // ~keep `[foo](uri2)`.
    #[test]
    fn escape_link_label_escapes_a_reference_link_shaped_bracket_pair() {
        assert_eq!(escape_link_label("[foo][ref]"), "\\[foo\\][ref]");
    }

    // ~keep A `[...]` NOT immediately followed by `(` or `[` cannot complete a link on
    // ~keep reparse, so it is left alone -- this is the exact shape the task's proposed
    // ~keep narrower rule ("escape `]` only before `(`") would also leave alone, but this
    // ~keep also confirms plain non-link-shaped bracket text is unaffected.
    #[test]
    fn escape_link_label_leaves_non_link_shaped_brackets_unchanged() {
        assert_eq!(escape_link_label("see [note] here"), "see [note] here");
    }

    // ~keep Regression: a link whose *label* is a real, intentionally-emitted nested
    // ~keep image (`![moon](moon.jpg)`, CommonMark permits images -- just not links --
    // ~keep inside link text) must not be escaped merely because it is link-shaped: it
    // ~keep is not literal text, it is markdown this converter already produced while
    // ~keep walking a nested `<img>`. Caught by `commonmark_compliance_test`'s example
    // ~keep 517 (`[![moon](moon.jpg)](/uri)`) before this exemption was added.
    #[test]
    fn escape_link_label_does_not_escape_a_nested_image() {
        assert_eq!(escape_link_label("![moon](moon.jpg)"), "![moon](moon.jpg)");
    }

    // ~keep A link-shaped bracket pair still nested inside an outer, non-link-shaped
    // ~keep bracket pair: only the inner (dangerous) pair is escaped, and the escape is
    // ~keep inserted at the correct byte offset for the *matching* opener, not just
    // ~keep prepended to the whole string.
    #[test]
    fn escape_link_label_escapes_only_the_link_shaped_inner_pair() {
        assert_eq!(escape_link_label("[a[b](c)]"), "[a\\[b\\](c)]");
    }
}
