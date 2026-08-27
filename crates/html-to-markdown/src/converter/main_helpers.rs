//! Helper functions for HTML to Markdown conversion.
//!
//! This module contains utility functions used by the main conversion pipeline,
//! including preprocessing helpers, HTML repair, and metadata formatting.

use std::collections::BTreeMap;

use crate::options::ConversionOptions;
use crate::options::conversion::{MAX_CONFIGURABLE_DEPTH, NATIVE_STACK_SAFE_DEPTH};

/// Resolve the effective traversal-depth ceiling.
///
/// When the caller does not set `max_depth`, the conservative native default is
/// used. An explicit `max_depth` is honored — allowing callers to raise the
/// ceiling above the native default (issue #434) — but is clamped to
/// [`MAX_CONFIGURABLE_DEPTH`] so the recursive walker cannot overflow the stack.
pub fn effective_max_depth(options: &ConversionOptions) -> usize {
    match options.max_depth {
        None => NATIVE_STACK_SAFE_DEPTH,
        Some(explicit) => explicit.min(MAX_CONFIGURABLE_DEPTH),
    }
}

/// Compare two tag names case-insensitively.
pub fn tag_name_eq(a: impl AsRef<str>, b: &str) -> bool {
    a.as_ref().eq_ignore_ascii_case(b)
}

/// Remove trailing spaces and tabs from a string.
pub fn trim_trailing_whitespace(output: &mut String) {
    while output.ends_with([' ', '\t']) {
        output.pop();
    }
}

/// Strip a trailing run of backslash hard-break markers from the end of a block's content.
///
/// CommonMark's hard-break rule has no effect at the end of a block: there is no next line
/// for the break to reach. For the two-space style the leftover marker is invisible trailing
/// whitespace, so it is harmless and left in place. For the backslash style the leftover `\`
/// is a literal, visible character, so a trailing run of one or more `<br>`-emitted `"\\\n"`
/// markers must be dropped entirely rather than rendered as stray backslashes (issue #464).
/// `block_start` bounds the strip to the current block so an earlier block's content already
/// in `output` is never touched.
pub fn strip_trailing_backslash_breaks(output: &mut String, block_start: usize) {
    while output.len() > block_start && output[block_start..].ends_with("\\\n") {
        let new_len = output.len() - "\\\n".len();
        output.truncate(new_len);
    }
}

/// Emit a line break for a `<br>`, `<div>`, or `<p>` continuation inside a table cell.
///
/// A Markdown table cell cannot contain a hard line break: neither `newline_style` form
/// (two-space or backslash) is valid there, and a raw newline corrupts the row by
/// splitting its pipe syntax across physical lines. So this always trims trailing
/// whitespace first, then emits a literal `<br>` when `br_in_tables` is true, or
/// collapses to a single space otherwise — `newline_style` is never consulted inside a
/// cell (issue #453, issue #454). The `output.is_empty()` guard suppresses a leading
/// space when the continuation is the first content in the cell; a leading `<br>` is
/// still emitted under `br_in_tables`, preserving the pre-existing `<br>` behaviour.
pub fn emit_table_cell_break(output: &mut String, br_in_tables: bool) {
    trim_trailing_whitespace(output);
    if br_in_tables {
        output.push_str("<br>");
    } else if !output.is_empty() {
        output.push(' ');
    }
}

/// Collapse runs of three or more consecutive newlines into exactly two.
///
/// Block-level emitters append their own trailing newlines and the next block
/// emitter typically prepends a leading newline, which can produce `\n\n\n`
/// runs in transitions such as frontmatter → first block or list → next block.
/// markdownlint's MD012 rule forbids multiple consecutive blank lines, so the
/// final emission is normalized here. This intentionally preserves single
/// blank lines (`\n\n`) — only runs of three or more newlines are collapsed.
pub fn collapse_excess_blank_lines(output: &mut String) {
    if !output.contains("\n\n\n") {
        return;
    }
    let mut cleaned = String::with_capacity(output.len());
    let mut consecutive = 0usize;
    for ch in output.chars() {
        if ch == '\n' {
            consecutive += 1;
            if consecutive <= 2 {
                cleaned.push(ch);
            }
        } else {
            consecutive = 0;
            cleaned.push(ch);
        }
    }
    *output = cleaned;
}

/// Remove trailing spaces/tabs from every line while preserving newlines.
pub fn trim_line_end_whitespace(output: &mut String) {
    if output.is_empty() {
        return;
    }

    let mut cleaned = String::with_capacity(output.len());
    for line in output.split('\n') {
        let content = line.trim_end_matches([' ', '\t']);
        cleaned.push_str(content);
        // ~keep The two-space hard break is only meaningful after content on the same line;
        // ~keep on a blank line CommonMark treats it as ordinary trailing whitespace. Keeping
        // ~keep it there made whitespace-only documents render as "  \n" instead of "".
        if !content.is_empty() && line.ends_with("  ") {
            cleaned.push_str("  ");
        }
        cleaned.push('\n');
    }

    let trimmed = cleaned.trim_end_matches('\n');
    if trimmed.is_empty() {
        *output = String::new();
    } else {
        if trimmed.len() < cleaned.len() {
            cleaned.truncate(trimmed.len() + 1);
        } else {
            cleaned.push('\n');
        }
        *output = cleaned;
    }
}

/// Check if HTML contains custom element tags.
pub fn has_custom_element_tags(html: &str) -> bool {
    // ~keep Custom elements must have a hyphen in their TAG NAME, not in attributes
    // ~keep Look for patterns like <foo-bar> or </foo-bar>
    let bytes = html.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        if bytes[i] == b'<' {
            i += 1;
            if i >= len {
                break;
            }

            if bytes[i] == b'/' {
                i += 1;
                if i >= len {
                    break;
                }
            }

            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }

            let tag_start = i;
            while i < len {
                let ch = bytes[i];
                if ch == b'>' || ch == b'/' || ch.is_ascii_whitespace() {
                    let tag_name = &bytes[tag_start..i];
                    if tag_name.contains(&b'-') {
                        return true;
                    }
                    break;
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    false
}

/// HTML5 void elements that are self-closing by spec and must NOT be expanded.
///
/// These elements are always void in HTML5: they have no end tag, and `<br />` is
/// equivalent to `<br>`.  We must leave them as-is when pre-processing XML-style
/// self-closing syntax so that `repair_with_html5ever` can parse them correctly.
const HTML5_VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr",
];

/// Expand XML-style self-closing tags to explicit open+close pairs.
///
/// HTML5 does not honour the `/>` self-close syntax for non-void elements.  When
/// `repair_with_html5ever` re-parses content that contains custom / namespaced tags
/// written as `<ac:parameter name="foo" />`, the HTML5 parser treats the `/>` as `>`
/// and leaves the element open.  Subsequent siblings then nest inside it, breaking
/// visitor pre-order/post-order start/end pairing.
///
/// This function scans the input byte-by-byte and rewrites any `<tag ... />` where
/// `tag` is not a known HTML5 void element into `<tag ...></tag>`.  Known void
/// elements are left unchanged because they must not receive an explicit close tag.
///
/// # Correctness guarantees
/// - Non-ASCII bytes are never interpreted as structural characters; all multi-byte
///   UTF-8 sequences pass through unmodified via `&input[byte_offset..]` slicing.
/// - Attribute values containing `/>` are skipped correctly (the scanner tracks
///   whether it is inside a quoted attribute).
/// - `</closing>` tags are never modified.
/// - The function is pure and returns a new `String`; if no substitution is needed
///   the allocation is still performed (cheap given repair is already rare).
pub fn expand_xml_self_closing_tags(input: &str) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut output = String::with_capacity(len);
    let mut copy_start = 0usize;
    let mut i = 0;

    while i < len {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }

        let tag_open = i;
        i += 1;

        if i < len && bytes[i] == b'/' {
            while i < len && bytes[i] != b'>' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            continue;
        }

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let name_start = i;
        while i < len {
            let ch = bytes[i];
            if ch == b'>' || ch == b'/' || ch.is_ascii_whitespace() {
                break;
            }
            i += 1;
        }
        let tag_name_bytes = &bytes[name_start..i];

        if tag_name_bytes.is_empty() {
            continue;
        }

        let tag_name_lower = tag_name_bytes.iter().map(u8::to_ascii_lowercase).collect::<Vec<_>>();
        let is_void = HTML5_VOID_ELEMENTS
            .iter()
            .any(|v| v.as_bytes() == tag_name_lower.as_slice());

        let attrs_start = i;
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut self_closing = false;

        while i < len {
            match bytes[i] {
                b'"' if !in_single_quote => {
                    in_double_quote = !in_double_quote;
                    i += 1;
                }
                b'\'' if !in_double_quote => {
                    in_single_quote = !in_single_quote;
                    i += 1;
                }
                b'/' if !in_single_quote && !in_double_quote => {
                    if i + 1 < len && bytes[i + 1] == b'>' {
                        self_closing = true;
                        break;
                    }
                    i += 1;
                }
                b'>' if !in_single_quote && !in_double_quote => {
                    break;
                }
                _ => {
                    i += 1;
                }
            }
        }

        if self_closing && !is_void {
            output.push_str(&input[copy_start..tag_open]);

            let tag_name_str = std::str::from_utf8(tag_name_bytes).unwrap_or("");
            let attrs_part = &input[attrs_start..i];

            output.push('<');
            output.push_str(tag_name_str);
            output.push_str(attrs_part);
            output.push('>');
            output.push('<');
            output.push('/');
            output.push_str(tag_name_str);
            output.push('>');

            i += 2;
            copy_start = i;
        } else {
            if i < len && bytes[i] == b'/' {
                i += 2;
            } else if i < len && bytes[i] == b'>' {
                i += 1;
            }
        }
    }

    output.push_str(&input[copy_start..]);
    output
}

/// Try to repair HTML using html5ever parser.
///
/// Returns `Some(repaired_html)` if repair was successful, None otherwise.
///
/// Before feeding the input to the HTML5 parser, XML-style self-closing tags on
/// non-void elements (e.g. `<ac:parameter name="foo" />`) are expanded to explicit
/// open+close pairs.  This preserves the intended document structure because HTML5
/// semantics do not honour `/>` on unknown elements — without the expansion, the
/// element would be left open and subsequent siblings would nest inside it, breaking
/// visitor start/end event pairing (issue #331).
pub fn repair_with_html5ever(input: &str) -> Option<String> {
    use crate::rcdom::{RcDom, SerializableHandle};
    use html5ever::serialize::{SerializeOpts, serialize};
    use html5ever::tendril::TendrilSink;

    let expanded = expand_xml_self_closing_tags(input);

    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut expanded.as_bytes())
        .ok()?;

    let mut buf = Vec::with_capacity(input.len());
    let handle = SerializableHandle::from(dom.document);
    serialize(&mut buf, &handle, SerializeOpts::default()).ok()?;
    String::from_utf8(buf).ok()
}

/// Format metadata as YAML frontmatter.
pub fn format_metadata_frontmatter(metadata: &BTreeMap<String, String>) -> String {
    let mut result = String::from("---\n");
    for (key, value) in metadata {
        use std::fmt::Write as _;
        let _ = writeln!(&mut result, "{key}: {value}");
    }
    result.push_str("---\n");
    result
}

/// Extract metadata from the head element.
pub fn extract_head_metadata(
    node_handle: &tl::NodeHandle,
    parser: &tl::Parser,
    options: &ConversionOptions,
) -> BTreeMap<String, String> {
    let mut work = vec![*node_handle];
    while let Some(handle) = work.pop() {
        let Some(tl::Node::Tag(tag)) = handle.get(parser) else {
            continue;
        };

        if !tag.name().as_utf8_str().eq_ignore_ascii_case("head") {
            let children: Vec<_> = tag.children().top().iter().copied().collect();
            for child_handle in children.into_iter().rev() {
                work.push(child_handle);
            }
            continue;
        }

        let mut metadata = BTreeMap::new();
        {
            let children = tag.children();
            for child_handle in children.top().iter() {
                if let Some(tl::Node::Tag(child_tag)) = child_handle.get(parser) {
                    if child_tag.name().as_utf8_str().eq_ignore_ascii_case("meta")
                        && !options.strip_tags.iter().any(|t| t == "meta")
                        && !options.preserve_tags.iter().any(|t| t == "meta")
                    {
                        if let (Some(name), Some(content)) = (
                            child_tag.attributes().get("name").flatten(),
                            child_tag.attributes().get("content").flatten(),
                        ) {
                            let name_str = name.as_utf8_str();
                            let content_str = content.as_utf8_str();
                            metadata.insert(format!("meta-{name_str}"), content_str.to_string());
                        }
                        if let (Some(property), Some(content)) = (
                            child_tag.attributes().get("property").flatten(),
                            child_tag.attributes().get("content").flatten(),
                        ) {
                            let property_str = property.as_utf8_str();
                            let content_str = content.as_utf8_str();
                            metadata.insert(format!("meta-{property_str}"), content_str.to_string());
                        }
                    }
                    if child_tag.name().as_utf8_str().eq_ignore_ascii_case("title")
                        && !options.strip_tags.iter().any(|t| t == "title")
                        && !options.preserve_tags.iter().any(|t| t == "title")
                    {
                        let mut title_content = String::new();
                        let title_children = child_tag.children();
                        for title_child in title_children.top().iter() {
                            if let Some(tl::Node::Raw(raw)) = title_child.get(parser) {
                                title_content.push_str(raw.as_utf8_str().as_ref());
                            }
                        }
                        title_content = title_content.trim().to_string();
                        if !title_content.is_empty() {
                            metadata.insert("title".to_string(), title_content);
                        }
                    }
                    if child_tag.name().as_utf8_str().eq_ignore_ascii_case("link") {
                        if let Some(rel_attr) = child_tag.attributes().get("rel").flatten() {
                            let rel_str = rel_attr.as_utf8_str();
                            if rel_str.contains("canonical") {
                                if let Some(href_attr) = child_tag.attributes().get("href").flatten() {
                                    let href_str = href_attr.as_utf8_str();
                                    metadata.insert("canonical".to_string(), href_str.to_string());
                                }
                            }
                        }
                    }
                    if child_tag.name().as_utf8_str().eq_ignore_ascii_case("base") {
                        if let Some(href_attr) = child_tag.attributes().get("href").flatten() {
                            let href_str = href_attr.as_utf8_str();
                            metadata.insert("base".to_string(), href_str.to_string());
                        }
                    }
                }
            }
        }

        if !metadata.is_empty() {
            return metadata;
        }
    }

    BTreeMap::new()
}

/// Check if text has more than one character.
pub fn has_more_than_one_char(text: &str) -> bool {
    let mut chars = text.chars();
    chars.next().is_some() && chars.next().is_some()
}

/// Check if an element is inline (not block-level).
pub fn is_inline_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "a" | "abbr"
            | "b"
            | "bdi"
            | "bdo"
            | "br"
            | "cite"
            | "code"
            | "data"
            | "dfn"
            | "em"
            | "i"
            | "kbd"
            | "mark"
            | "q"
            | "rp"
            | "rt"
            | "ruby"
            | "s"
            | "samp"
            | "small"
            | "span"
            | "strong"
            | "sub"
            | "sup"
            | "time"
            | "u"
            | "var"
            | "wbr"
            | "del"
            | "ins"
            | "img"
            | "map"
            | "area"
            | "audio"
            | "video"
            | "picture"
            | "source"
            | "track"
            | "embed"
            | "object"
            | "param"
            | "input"
            | "label"
            | "button"
            | "select"
            | "textarea"
            | "output"
            | "progress"
            | "meter"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_line_end_whitespace() {
        let mut s = String::new();
        trim_line_end_whitespace(&mut s);
        assert_eq!("", s.as_str());

        let mut s = "\t\n\t\n".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("", s.as_str());

        let mut s = "hello, world  ".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n", s.as_str());

        let mut s = "hello, world  \n".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n", s.as_str());

        let mut s = "hello, world  ".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n", s.as_str());

        let mut s = "hello, world  \n\n\n".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n", s.as_str());

        let mut s = "hello  \n- world\n".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello  \n- world\n", s.as_str());

        let mut s = "hello, world\t\t  ".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n", s.as_str());

        let mut s = "hello, world\t\t  \n.abc def \t \t".to_owned();
        trim_line_end_whitespace(&mut s);
        assert_eq!("hello, world  \n.abc def\n", s.as_str());
    }
}
