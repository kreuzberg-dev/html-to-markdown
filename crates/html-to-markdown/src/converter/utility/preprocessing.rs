//! HTML preprocessing and normalization.
//!
//! Functions for preprocessing HTML before conversion, including script/style stripping,
//! tag repair, and malformed HTML handling.

use std::borrow::Cow;
use std::str;

/// Strip script and style tags and their content from HTML.
pub fn strip_script_and_style_tags(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;
    let mut svg_depth = 0usize;

    if !bytes.contains(&b'<') {
        return Cow::Borrowed(input);
    }

    while idx < len {
        if bytes[idx] == b'<' && idx + 1 < len {
            if matches_tag_start(bytes, idx + 1, b"svg") {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + b"svg".len()) {
                    svg_depth += 1;
                    idx = open_end;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, b"svg") {
                if let Some(close_end) = find_tag_end(bytes, idx + 2 + b"svg".len()) {
                    if svg_depth > 0 {
                        svg_depth = svg_depth.saturating_sub(1);
                    }
                    idx = close_end;
                    continue;
                }
            }

            if svg_depth > 0 {
                idx += 1;
                continue;
            }

            // ~keep Check for </script or </style (closing tags first for safety)
            if bytes[idx + 1] == b'/' && idx + 2 < len {
                if idx + 9 <= len && eq_ascii_insensitive(&bytes[idx..idx + 9], b"</script>") {
                    idx += 9;
                    continue;
                }

                if idx + 8 <= len && eq_ascii_insensitive(&bytes[idx..idx + 8], b"</style>") {
                    idx += 8;
                    continue;
                }
            }

            if idx + 7 < len && eq_ascii_insensitive(&bytes[idx..idx + 7], b"<script") {
                let after_tag = bytes[idx + 7];
                if after_tag == b'>'
                    || after_tag == b' '
                    || after_tag == b'\t'
                    || after_tag == b'\n'
                    || after_tag == b'\r'
                {
                    let mut tag_end = idx + 7;
                    while tag_end < len && bytes[tag_end] != b'>' {
                        tag_end += 1;
                    }

                    if tag_end < len {
                        tag_end += 1;

                        let tag_content = &input[idx..tag_end];
                        if !is_json_ld_script_open_tag(tag_content) {
                            let close_tag = find_closing_tag_bytes(bytes, tag_end, b"script");
                            if let Some(close_idx) = close_tag {
                                let out = output.get_or_insert_with(|| String::with_capacity(len));
                                out.push_str(&input[last..idx]);
                                if idx > 0
                                    && close_idx < len
                                    && !bytes[idx - 1].is_ascii_whitespace()
                                    && !bytes[close_idx].is_ascii_whitespace()
                                {
                                    out.push(' ');
                                }
                                last = close_idx;
                                idx = close_idx;
                                continue;
                            }
                        }
                    }
                }
            } else if idx + 6 < len && eq_ascii_insensitive(&bytes[idx..idx + 6], b"<style") {
                let after_tag = bytes[idx + 6];
                if after_tag == b'>'
                    || after_tag == b' '
                    || after_tag == b'\t'
                    || after_tag == b'\n'
                    || after_tag == b'\r'
                {
                    let mut tag_end = idx + 6;
                    while tag_end < len && bytes[tag_end] != b'>' {
                        tag_end += 1;
                    }

                    if tag_end < len {
                        tag_end += 1;

                        let close_tag = find_closing_tag_bytes(bytes, tag_end, b"style");
                        if let Some(close_idx) = close_tag {
                            let out = output.get_or_insert_with(|| String::with_capacity(len));
                            out.push_str(&input[last..idx]);
                            if idx > 0
                                && close_idx < len
                                && !bytes[idx - 1].is_ascii_whitespace()
                                && !bytes[close_idx].is_ascii_whitespace()
                            {
                                out.push(' ');
                            }
                            last = close_idx;
                            idx = close_idx;
                            continue;
                        }
                    }
                }
            }
        }

        idx += 1;
    }

    if let Some(mut out) = output {
        if last < len {
            out.push_str(&input[last..]);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(input)
    }
}

/// Upper bound on how far a closing-tag scan may travel from its start offset.
///
/// ~keep DoS guard: hostile input can contain an unterminated element followed
/// ~keep by hundreds of megabytes of text; the scan gives up rather than walking
/// ~keep the whole document for every such tag.
const MAX_CLOSING_TAG_SCAN: usize = 100_000_000;

/// Find the position of the FIRST closing tag in bytes, ignoring nesting.
/// Returns the position AFTER the closing tag (including the '>').
/// This is highly optimized for performance and uses a fast-path scan.
///
/// ~keep First-match (not depth-aware) is correct ONLY for the HTML raw-text
/// ~keep elements `<script>` and `<style>`: inside their bodies `<script` is
/// ~keep text, not a tag, so the first `</script>` is the spec terminator.
/// ~keep Counting depth here would treat `var s = "<script>"` as an opening tag
/// ~keep and run past the real close, swallowing the rest of the document.
/// ~keep Elements that can nest must use `find_closing_tag_bytes_nested`; do not
/// ~keep unify the two functions.
#[inline]
pub fn find_closing_tag_bytes(bytes: &[u8], start: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let tag_len = tag.len();

    let mut idx = start;

    while idx < len && (idx - start) < MAX_CLOSING_TAG_SCAN {
        if bytes[idx] != b'<' {
            if let Some(pos) = memchr::memchr(b'<', &bytes[idx..]) {
                idx += pos;
            } else {
                break;
            }
        }

        if idx + 2 < len && bytes[idx + 1] == b'/' {
            if idx + 2 + tag_len <= len && eq_ascii_insensitive(&bytes[idx + 2..idx + 2 + tag_len], tag) {
                let after_tag = idx + 2 + tag_len;
                if after_tag < len && (bytes[after_tag] == b'>' || bytes[after_tag].is_ascii_whitespace()) {
                    let mut close_idx = after_tag;
                    while close_idx < len && bytes[close_idx] != b'>' {
                        close_idx += 1;
                    }
                    if close_idx < len {
                        return Some(close_idx + 1);
                    }
                }
            }
        }

        idx += 1;
    }

    None
}

/// Void elements that never carry a closing tag, so an occurrence of one inside
/// a subtree must not raise the nesting depth of a closing-tag scan.
const VOID_ELEMENT_NAMES: [&[u8]; 4] = [b"br", b"hr", b"img", b"input"];

/// Whether an opening tag closes itself: XHTML-style `<tag/>` or a void element.
///
/// ~keep Shared by `strip_hidden_elements` and `find_closing_tag_bytes_nested`
/// ~keep so the element the stripper treats as self-closing and the element the
/// ~keep depth counter refuses to count can never disagree.
#[inline]
fn is_self_closing_tag(tag_slice: &[u8], tag_name: &[u8]) -> bool {
    tag_slice.ends_with(b"/>")
        || VOID_ELEMENT_NAMES
            .iter()
            .any(|void_name| tag_name.eq_ignore_ascii_case(void_name))
}

/// Byte sequences delimiting the regions of an HTML document whose bytes are
/// character data rather than markup.
const COMMENT_OPEN: &[u8] = b"<!--";
const COMMENT_CLOSE: &[u8] = b"-->";
const CDATA_OPEN: &[u8] = b"<![CDATA[";
const CDATA_CLOSE: &[u8] = b"]]>";

/// Elements whose body is raw text: `<` inside them never starts a tag.
const RAW_TEXT_ELEMENT_NAMES: [&[u8]; 2] = [b"script", b"style"];

/// Index just past the first occurrence of `terminator` at or after `from`,
/// or the input length when the region is never terminated.
///
/// ~keep An unterminated comment/CDATA region runs to EOF per HTML5, so
/// ~keep returning `len` (rather than `None`) keeps the caller from resuming a
/// ~keep tag scan inside text that will never be markup.
#[inline]
fn find_sequence_end(bytes: &[u8], from: usize, terminator: &[u8]) -> usize {
    let len = bytes.len();
    if from >= len {
        return len;
    }
    match bytes[from..].windows(terminator.len()).position(|w| w == terminator) {
        Some(offset) => from + offset + terminator.len(),
        None => len,
    }
}

/// If a `<` at `idx` opens a region that is NOT markup — an HTML comment, a
/// CDATA section, or the body of a raw-text element — return the index just
/// past that region. Returns `None` when `idx` does not open such a region.
///
/// ~keep A depth-counting close-tag scan MUST consult this before interpreting
/// ~keep a `<`: `<!-- </div> -->` and `<script>{"a":"</div>"}</script>` contain
/// ~keep byte sequences that look like tags but are text. Counting them ends the
/// ~keep scan early (leaking the hidden element's tail into the output) or
/// ~keep inflates the depth (swallowing the following visible sibling).
#[inline]
pub fn skip_opaque_region(bytes: &[u8], idx: usize) -> Option<usize> {
    let len = bytes.len();
    if idx >= len || bytes[idx] != b'<' {
        return None;
    }

    if bytes[idx..].starts_with(COMMENT_OPEN) {
        return Some(find_sequence_end(bytes, idx + COMMENT_OPEN.len(), COMMENT_CLOSE));
    }

    if bytes[idx..].starts_with(CDATA_OPEN) {
        return Some(find_sequence_end(bytes, idx + CDATA_OPEN.len(), CDATA_CLOSE));
    }

    for raw_text_name in RAW_TEXT_ELEMENT_NAMES {
        if matches_tag_start(bytes, idx + 1, raw_text_name) {
            let open_end = find_tag_end(bytes, idx + 1 + raw_text_name.len())?;
            // ~keep First-match close is the spec terminator for raw text — see
            // ~keep the doc comment on `find_closing_tag_bytes`.
            return Some(find_closing_tag_bytes(bytes, open_end, raw_text_name).unwrap_or(len));
        }
    }

    None
}

/// Whether the `<` at `idx` starts a tag rather than a stray `<` in text.
///
/// ~keep Mirrors the `is_valid_tag` test in `preprocess_html`: without it a
/// ~keep literal `<` in text (`a < b`) would make a scan jump to the next `>`,
/// ~keep stepping over the real closing tag in between.
#[inline]
fn opens_a_tag(bytes: &[u8], idx: usize) -> bool {
    match bytes.get(idx + 1) {
        Some(b'/' | b'!') => bytes.get(idx + 2).is_some_and(u8::is_ascii_alphabetic),
        Some(byte) => byte.is_ascii_alphabetic(),
        None => false,
    }
}

/// Find the closing tag that matches an opening `<tag>`, counting nested
/// elements of the same name. Returns the position AFTER the closing tag
/// (including the '>'), or `None` if the element is never closed.
///
/// ~keep Required for elements that can nest (`div`, `span`, ...): the
/// ~keep first-match `find_closing_tag_bytes` stops at the INNER `</tag>`, which
/// ~keep leaves the hidden element's tail in the document. Kept separate from
/// ~keep that function because raw-text elements must NOT count depth — see its
/// ~keep doc comment.
///
/// ~keep Only real markup may move the depth counter, so every `<` is first
/// ~keep offered to `skip_opaque_region`, and a tag that is not the target is
/// ~keep jumped via the quote-aware `find_tag_end` rather than byte-by-byte:
/// ~keep otherwise `</tag>` written inside a comment, a quoted attribute value,
/// ~keep or a raw-text body is mistaken for the element's terminator.
#[inline]
pub fn find_closing_tag_bytes_nested(bytes: &[u8], start: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    if tag.is_empty() {
        return None;
    }

    let mut idx = start;
    let mut depth = 1usize;

    while idx < len && (idx - start) < MAX_CLOSING_TAG_SCAN {
        if bytes[idx] != b'<' {
            match memchr::memchr(b'<', &bytes[idx..]) {
                Some(pos) => idx += pos,
                None => break,
            }
        }

        if let Some(region_end) = skip_opaque_region(bytes, idx) {
            idx = region_end;
            continue;
        }

        if matches_end_tag_start(bytes, idx + 1, tag) {
            if let Some(close_end) = find_tag_end(bytes, idx + 2 + tag.len()) {
                depth -= 1;
                if depth == 0 {
                    return Some(close_end);
                }
                idx = close_end;
                continue;
            }
        } else if matches_tag_start(bytes, idx + 1, tag) {
            if let Some(open_end) = find_tag_end(bytes, idx + 1 + tag.len()) {
                if !is_self_closing_tag(&bytes[idx..open_end], tag) {
                    depth += 1;
                }
                idx = open_end;
                continue;
            }
        } else if opens_a_tag(bytes, idx) {
            if let Some(unrelated_tag_end) = find_tag_end(bytes, idx + 1) {
                idx = unrelated_tag_end;
                continue;
            }
        }

        idx += 1;
    }

    None
}

/// Compare bytes ignoring ASCII case.
#[inline]
pub fn eq_ascii_insensitive(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}

/// Normalize HTML comment endings that would confuse the `tl` parser.
///
/// The `astral-tl` parser mishandles HTML comments whose closing sequence
/// contains more than two dashes before the `>` (e.g. `<!-- foo --->` or
/// `<!-- foo ---->`).  When it encounters such a comment it creates an empty
/// comment node and silently discards every byte that follows, so all document
/// content after the comment is lost.
///
/// This function rewrites those bogus closings: every `--[-]+>` sequence that
/// terminates an HTML comment is normalised to `-->`.  Regular `-->` closings
/// are left unchanged.
///
/// # Algorithm
///
/// Scans the input byte-by-byte looking for `<!--`.  For each comment found it
/// scans forward for `-->` using the HTML5 comment-end state machine:
///
/// - `--[` zero or more `-` `]>` ends the comment.
/// - Any other character after `--` resets back into the comment body.
///
/// If the actual number of leading dashes before `>` is more than two the
/// closing sequence is replaced with `-->`.
pub fn normalize_bogus_comment_endings(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len < 7 || !bytes.windows(4).any(|w| w == b"<!--") {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx + 3 < len {
        if !(bytes[idx] == b'<' && bytes[idx + 1] == b'!' && bytes[idx + 2] == b'-' && bytes[idx + 3] == b'-') {
            idx += 1;
            continue;
        }

        idx += 4;

        let mut consecutive_dashes: usize = 0;

        while idx < len {
            let b = bytes[idx];
            if b == b'-' {
                consecutive_dashes += 1;
                idx += 1;
            } else if b == b'>' && consecutive_dashes >= 2 {
                if consecutive_dashes > 2 {
                    let out = output.get_or_insert_with(|| String::with_capacity(len));
                    let close_start = idx - consecutive_dashes;
                    out.push_str(&input[last..close_start]);
                    out.push_str("-->");
                    idx += 1;
                    last = idx;
                } else {
                    idx += 1;
                }
                break;
            } else {
                consecutive_dashes = 0;
                idx += 1;
            }
        }
    }

    match output {
        Some(mut out) => {
            if last < len {
                out.push_str(&input[last..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Normalize closing tags whose `>` appears on a subsequent line.
///
/// Some HTML formatters (JSX-style) write closing tags as:
///
/// ```html
/// </a
/// >
/// ```
///
/// The `tl` parser does not handle end-tags with a newline before the closing
/// `>`, leaving the element unclosed so all subsequent siblings become children
/// of the open element.  This pass collapses such patterns to a single-line
/// closing tag (`</a>`) before the document reaches `tl`.
///
/// Only the whitespace between the tag name and the closing `>` is normalised;
/// the rest of the document is untouched.
pub fn normalize_split_closing_tags(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len < 4 || !bytes.contains(&b'\n') {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx + 2 < len {
        if bytes[idx] != b'<' || bytes[idx + 1] != b'/' {
            idx += 1;
            continue;
        }

        // ~keep Scan tag name: ASCII letters, digits, hyphens (HTML5 allows hyphens in custom elements)
        let name_start = idx + 2;
        let mut name_end = name_start;
        while name_end < len && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'-') {
            name_end += 1;
        }

        if name_end == name_start {
            idx += 1;
            continue;
        }

        let ws_start = name_end;
        let mut ws_end = ws_start;
        let mut has_newline = false;
        while ws_end < len && bytes[ws_end].is_ascii_whitespace() {
            if bytes[ws_end] == b'\n' || bytes[ws_end] == b'\r' {
                has_newline = true;
            }
            ws_end += 1;
        }

        if !has_newline || ws_end >= len || bytes[ws_end] != b'>' {
            idx += 1;
            continue;
        }

        let tag_name = &input[name_start..name_end];
        let out = output.get_or_insert_with(|| String::with_capacity(len));
        out.push_str(&input[last..idx]);
        out.push_str("</");
        out.push_str(tag_name);
        out.push('>');

        idx = ws_end + 1;
        last = idx;
    }

    match output {
        Some(mut out) => {
            if last < len {
                out.push_str(&input[last..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Preprocess HTML to normalize tags and fix common issues.
pub fn preprocess_html(input: &str) -> Cow<'_, str> {
    const SELF_CLOSING: [(&[u8], &str); 3] = [(b"<br/>", "<br>"), (b"<hr/>", "<hr>"), (b"<img/>", "<img>")];
    const TAGS: [&[u8]; 2] = [b"script", b"style"];
    const SVG: &[u8] = b"svg";
    const DOCTYPE: &[u8] = b"doctype";
    const EMPTY_COMMENT: &[u8] = b"<!---->";

    let bytes = input.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return Cow::Borrowed(input);
    }

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;
    let mut svg_depth = 0usize;

    while idx < len {
        if bytes[idx] == b'<' {
            if bytes[idx..].starts_with(EMPTY_COMMENT) {
                let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                out.push_str(&input[last..idx]);
                out.push_str("<!-- -->");
                idx += EMPTY_COMMENT.len();
                last = idx;
                continue;
            }

            let mut replaced = false;
            for (pattern, replacement) in &SELF_CLOSING {
                if bytes[idx..].starts_with(pattern) {
                    let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                    out.push_str(&input[last..idx]);
                    out.push_str(replacement);
                    idx += pattern.len();
                    last = idx;
                    replaced = true;
                    break;
                }
            }
            if replaced {
                continue;
            }

            if matches_tag_start(bytes, idx + 1, SVG) {
                if let Some(open_end) = find_tag_end(bytes, idx + 1 + SVG.len()) {
                    svg_depth += 1;
                    idx = open_end;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, SVG) {
                if let Some(close_end) = find_tag_end(bytes, idx + 2 + SVG.len()) {
                    if svg_depth > 0 {
                        svg_depth = svg_depth.saturating_sub(1);
                    }
                    idx = close_end;
                    continue;
                }
            }

            if svg_depth == 0 {
                let mut handled = false;
                for tag in TAGS {
                    if matches_tag_start(bytes, idx + 1, tag) {
                        if let Some(open_end) = find_tag_end(bytes, idx + 1 + tag.len()) {
                            if tag == b"script" && is_json_ld_script_open_tag(&input[idx..open_end]) {
                                continue;
                            }
                            let remove_end = find_closing_tag(bytes, open_end, tag).unwrap_or(open_end);
                            let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                            out.push_str(&input[last..idx]);
                            out.push_str(&input[idx..open_end]);
                            out.push_str("</");
                            if let Ok(tag_str) = str::from_utf8(tag) {
                                out.push_str(tag_str);
                            }
                            out.push('>');

                            last = remove_end;
                            idx = remove_end;
                            handled = true;
                        }
                    }

                    if handled {
                        break;
                    }
                }

                if handled {
                    continue;
                }

                if idx + 2 < len && bytes[idx + 1] == b'!' {
                    let mut cursor = idx + 2;
                    while cursor < len && bytes[cursor].is_ascii_whitespace() {
                        cursor += 1;
                    }

                    if cursor + DOCTYPE.len() <= len
                        && bytes[cursor..cursor + DOCTYPE.len()].eq_ignore_ascii_case(DOCTYPE)
                    {
                        if let Some(end) = find_tag_end(bytes, cursor + DOCTYPE.len()) {
                            let out = output.get_or_insert_with(|| String::with_capacity(input.len()));
                            out.push_str(&input[last..idx]);
                            last = end;
                            idx = end;
                            continue;
                        }
                    }
                }
            }

            let is_valid_tag = if idx + 1 < len {
                match bytes[idx + 1] {
                    b'!' => {
                        idx + 2 < len
                            && (bytes[idx + 2] == b'-'
                                || bytes[idx + 2].is_ascii_alphabetic()
                                || bytes[idx + 2].is_ascii_uppercase())
                    }
                    b'/' => {
                        idx + 2 < len && (bytes[idx + 2].is_ascii_alphabetic() || bytes[idx + 2].is_ascii_uppercase())
                    }
                    b'?' => true,
                    c if c.is_ascii_alphabetic() || c.is_ascii_uppercase() => true,
                    _ => false,
                }
            } else {
                false
            };

            if !is_valid_tag {
                let out = output.get_or_insert_with(|| String::with_capacity(input.len() + 4));
                out.push_str(&input[last..idx]);
                out.push_str("&lt;");
                idx += 1;
                last = idx;
                continue;
            }
        }

        idx += 1;
    }

    if let Some(mut out) = output {
        if last < len {
            out.push_str(&input[last..]);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(input)
    }
}

/// Check if a script tag is a JSON-LD script.
pub fn is_json_ld_script_open_tag(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    let mut idx = 0;
    while idx + 4 <= bytes.len() {
        if eq_ascii_case_insensitive(&bytes[idx..], b"type") {
            let before_ok = idx == 0
                || bytes
                    .get(idx.saturating_sub(1))
                    .is_some_and(|b| b.is_ascii_whitespace() || *b == b'<' || *b == b'/');
            let after_ok = bytes
                .get(idx + 4)
                .is_some_and(|b| b.is_ascii_whitespace() || *b == b'=');
            if !before_ok || !after_ok {
                idx += 4;
                continue;
            }

            let mut i = idx + 4;
            while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
                i += 1;
            }
            if bytes.get(i) != Some(&b'=') {
                idx += 4;
                continue;
            }
            i += 1;
            while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
                i += 1;
            }
            if i >= bytes.len() {
                return false;
            }

            let (value_start, value_end) = match bytes[i] {
                b'"' | b'\'' => {
                    let quote = bytes[i];
                    let start = i + 1;
                    let mut end = start;
                    while end < bytes.len() && bytes[end] != quote {
                        end += 1;
                    }
                    (start, end)
                }
                _ => {
                    let start = i;
                    let mut end = start;
                    while end < bytes.len() && !bytes[end].is_ascii_whitespace() && bytes[end] != b'>' {
                        end += 1;
                    }
                    (start, end)
                }
            };

            let value = &tag[value_start..value_end];
            let media_type = value.split(';').next().unwrap_or(value).trim();
            return eq_ascii_case_insensitive(media_type.as_bytes(), b"application/ld+json");
        }
        idx += 1;
    }
    false
}

/// Case-insensitive byte comparison for ASCII.
#[inline]
pub fn eq_ascii_case_insensitive(haystack: &[u8], needle: &[u8]) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    haystack
        .iter()
        .zip(needle.iter())
        .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

/// Check if bytes match a tag start pattern.
pub fn matches_tag_start(bytes: &[u8], mut start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() {
        return false;
    }

    if start + tag.len() > bytes.len() {
        return false;
    }

    if !bytes[start..start + tag.len()].eq_ignore_ascii_case(tag) {
        return false;
    }

    start += tag.len();

    match bytes.get(start) {
        Some(b'>' | b'/' | b' ' | b'\t' | b'\n' | b'\r') => true,
        Some(_) => false,
        None => true,
    }
}

/// Find the end of an HTML tag (the position of '>').
pub fn find_tag_end(bytes: &[u8], mut idx: usize) -> Option<usize> {
    let len = bytes.len();
    let mut in_quote: Option<u8> = None;

    while idx < len {
        match bytes[idx] {
            b'"' | b'\'' => {
                if let Some(current) = in_quote {
                    if current == bytes[idx] {
                        in_quote = None;
                    }
                } else {
                    in_quote = Some(bytes[idx]);
                }
            }
            b'>' if in_quote.is_none() => return Some(idx + 1),
            _ => {}
        }
        idx += 1;
    }

    None
}

/// Find the closing tag for a given tag name.
pub fn find_closing_tag(bytes: &[u8], mut idx: usize, tag: &[u8]) -> Option<usize> {
    let len = bytes.len();
    let mut depth = 1usize;

    while idx < len {
        if bytes[idx] == b'<' {
            if matches_tag_start(bytes, idx + 1, tag) {
                if let Some(next) = find_tag_end(bytes, idx + 1 + tag.len()) {
                    depth += 1;
                    idx = next;
                    continue;
                }
            } else if matches_end_tag_start(bytes, idx + 1, tag) {
                if let Some(close) = find_tag_end(bytes, idx + 2 + tag.len()) {
                    depth -= 1;
                    if depth == 0 {
                        return Some(close);
                    }
                    idx = close;
                    continue;
                }
            }
        }

        idx += 1;
    }

    None
}

/// Check if bytes match an end tag pattern.
pub fn matches_end_tag_start(bytes: &[u8], start: usize, tag: &[u8]) -> bool {
    if start >= bytes.len() || bytes[start] != b'/' {
        return false;
    }
    matches_tag_start(bytes, start + 1, tag)
}

/// Close implicitly-terminated list-item elements that `tl` would otherwise
/// absorb as deep children, causing stack overflows on large documents.
///
/// The HTML5 parsing spec (§13.2.6.4.7 "in body" insertion mode) states that
/// an open `<li>`, `<dt>`, or `<dd>` tag is *implicitly closed* when:
///
/// - Another `<li>` / `<dt>` / `<dd>` open tag is encountered, or
/// - The closing `</ul>`, `</ol>`, or `</dl>` tag is reached.
///
/// The `tl` parser is not a full HTML5 parser; it does not apply implicit
/// closure rules.  When it encounters `<li>content<li>more` it treats the
/// second `<li>` as a child of the first, producing a linear chain of depth
/// equal to the number of items.  A list with 400 unclosed `<li>` items
/// causes `walk_node` to recurse 400 levels deep; with 467 such lists in a
/// single document (curl.se/changes.html) the cumulative stack depth triggers
/// an OS-level stack overflow.
///
/// This function rewrites the source HTML in one linear pass before `tl` sees
/// it, inserting the missing `</li>`, `</dt>`, and `</dd>` close tags exactly
/// where the HTML5 spec says they belong.
///
/// # Scope
///
/// Only the three list-item element types are handled here; `<p>` and other
/// auto-closing block elements are intentionally left to the existing
/// `has_inline_block_misnest` → `repair_with_html5ever` path.
pub fn normalize_unclosed_list_items(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    if len < 4
        || (!bytes.windows(3).any(|w| {
            w.eq_ignore_ascii_case(b"<li") || w.eq_ignore_ascii_case(b"<dt") || w.eq_ignore_ascii_case(b"<dd")
        }))
    {
        return Cow::Borrowed(input);
    }

    let mut open_item: Option<&'static str> = None;
    let mut list_stack: Vec<Option<&'static str>> = Vec::new();

    let mut in_pre_or_code: usize = 0;
    let mut in_comment = false;

    let mut idx = 0usize;
    let mut last_flush = 0usize;
    let mut output: Option<String> = None;

    macro_rules! emit_close_before {
        ($pos:expr, $close_tag:expr) => {{
            let out = output.get_or_insert_with(|| String::with_capacity(len + 64));
            out.push_str(&input[last_flush..$pos]);
            out.push_str($close_tag);
            last_flush = $pos;
        }};
    }

    while idx < len {
        let b = bytes[idx];

        if in_comment {
            if b == b'-' && idx + 2 < len && bytes[idx + 1] == b'-' && bytes[idx + 2] == b'>' {
                in_comment = false;
                idx += 3;
            } else {
                idx += 1;
            }
            continue;
        }

        if b == b'<' && idx + 3 < len && bytes[idx + 1] == b'!' && bytes[idx + 2] == b'-' && bytes[idx + 3] == b'-' {
            in_comment = true;
            idx += 4;
            continue;
        }

        if b != b'<' {
            idx += 1;
            continue;
        }

        let tag_start = idx;
        idx += 1;
        if idx >= len {
            break;
        }

        let is_close = bytes[idx] == b'/';
        if is_close {
            idx += 1;
            if idx >= len {
                break;
            }
        }

        while idx < len && bytes[idx].is_ascii_whitespace() {
            idx += 1;
        }

        let name_start = idx;
        while idx < len {
            let ch = bytes[idx];
            if ch == b'>' || ch == b'/' || ch.is_ascii_whitespace() {
                break;
            }
            idx += 1;
        }
        let name_bytes = &bytes[name_start..idx];
        if name_bytes.is_empty() {
            continue;
        }

        {
            let mut in_single_quote = false;
            let mut in_double_quote = false;
            while idx < len {
                match bytes[idx] {
                    b'\'' if !in_double_quote => {
                        in_single_quote = !in_single_quote;
                        idx += 1;
                    }
                    b'"' if !in_single_quote => {
                        in_double_quote = !in_double_quote;
                        idx += 1;
                    }
                    b'>' if !in_single_quote && !in_double_quote => {
                        idx += 1;
                        break;
                    }
                    _ => {
                        idx += 1;
                    }
                }
            }
        }

        let tag_is_verbatim = name_bytes.eq_ignore_ascii_case(b"pre")
            || name_bytes.eq_ignore_ascii_case(b"code")
            || name_bytes.eq_ignore_ascii_case(b"script")
            || name_bytes.eq_ignore_ascii_case(b"style");

        if tag_is_verbatim {
            if is_close {
                in_pre_or_code = in_pre_or_code.saturating_sub(1);
            } else {
                in_pre_or_code += 1;
            }
            continue;
        }

        if in_pre_or_code > 0 {
            continue;
        }

        let is_list_container = name_bytes.eq_ignore_ascii_case(b"ul")
            || name_bytes.eq_ignore_ascii_case(b"ol")
            || name_bytes.eq_ignore_ascii_case(b"dl");

        let is_li = name_bytes.eq_ignore_ascii_case(b"li");
        let is_def_term = name_bytes.eq_ignore_ascii_case(b"dt");
        let is_def_desc = name_bytes.eq_ignore_ascii_case(b"dd");
        let is_list_item = is_li || is_def_term || is_def_desc;

        if is_close {
            if is_list_container {
                if let Some(item) = open_item.take() {
                    let close_tag = match item {
                        "li" => "</li>",
                        "dt" => "</dt>",
                        "dd" => "</dd>",
                        _ => unreachable!(),
                    };
                    emit_close_before!(tag_start, close_tag);
                }
                open_item = list_stack.pop().unwrap_or(None);
            } else if is_list_item {
                open_item = None;
            }
        } else {
            if is_list_container {
                list_stack.push(open_item.take());
            } else if is_list_item {
                let item_name: &'static str = if is_li {
                    "li"
                } else if is_def_term {
                    "dt"
                } else {
                    "dd"
                };

                if let Some(prev_item) = open_item.replace(item_name) {
                    let close_tag = match prev_item {
                        "li" => "</li>",
                        "dt" => "</dt>",
                        "dd" => "</dd>",
                        _ => unreachable!(),
                    };
                    emit_close_before!(tag_start, close_tag);
                }
            }
        }
    }

    if let Some(item) = open_item.take() {
        let close_tag = match item {
            "li" => "</li>",
            "dt" => "</dt>",
            "dd" => "</dd>",
            _ => unreachable!(),
        };
        let out = output.get_or_insert_with(|| String::with_capacity(len + 16));
        out.push_str(&input[last_flush..]);
        out.push_str(close_tag);
        last_flush = len;
    }

    match output {
        Some(mut out) => {
            if last_flush < len {
                out.push_str(&input[last_flush..]);
            }
            Cow::Owned(out)
        }
        None => Cow::Borrowed(input),
    }
}

/// Sanitize malformed markdown-like URLs in HTML attributes.
///
/// Handles cases like: `//[domain.com/path](http://domain.com/path)`
/// Extracts the actual URL from parentheses.
///
/// This is an internal function used during preprocessing to extract valid URLs
/// from malformed HTML that contains markdown-like syntax.
///
/// # Arguments
/// * `url` - The URL string to sanitize
///
/// # Returns
/// * `Cow<str>` - Either the borrowed original URL or an owned sanitized version
pub fn sanitize_markdown_url(url: &str) -> Cow<'_, str> {
    let Some(mid) = url.find("](") else {
        return Cow::Borrowed(url);
    };

    if !url[..mid].contains('[') {
        return Cow::Borrowed(url);
    }

    let paren_start = mid + 2;
    let Some(rel_end) = url[paren_start..].find(')') else {
        return Cow::Borrowed(url);
    };
    let paren_end = paren_start + rel_end;
    if paren_start >= paren_end {
        return Cow::Borrowed(url);
    }

    Cow::Owned(url[paren_start..paren_end].to_string())
}

/// Strip elements that are never rendered from HTML: those carrying the `hidden`
/// attribute, and those hidden via an inline `style="display:none"` or
/// `style="visibility:hidden"` declaration.
///
/// Scans for opening tags matching either condition, finds their matching
/// closing tag, and removes the entire element (tag + content, so nested
/// content never leaks out). Self-closing tags are also removed.
pub fn strip_hidden_elements(input: &str) -> Cow<'_, str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    if len == 0 || !bytes.contains(&b'<') {
        return Cow::Borrowed(input);
    }

    // ~keep DoS guard: a run of unterminated `<` (no `>` anywhere in the rest of the
    // ~keep document) makes `find_tag_end` scan to EOF on every single one, turning this
    // ~keep loop quadratic. Once no `>` remains past `idx`, `find_tag_end` is guaranteed
    // ~keep to fail regardless of quoting, so `last_gt` lets every remaining iteration
    // ~keep skip the call in O(1) instead of re-scanning to the end each time.
    let last_gt = bytes.iter().rposition(|&b| b == b'>');

    let mut idx = 0;
    let mut last = 0;
    let mut output: Option<String> = None;

    while idx < len {
        // ~keep A `<` not immediately followed by an ASCII letter can never start a real
        // ~keep HTML tag name (HTML5 tokenizer "tag open state"), so it is never worth a
        // ~keep `find_tag_end` scan. Without this, a run like `<<<<<` treats every `<` as a
        // ~keep candidate tag start, and each failing scan re-walks the same suffix.
        let starts_tag_name = idx + 1 < len && bytes[idx + 1].is_ascii_alphabetic();
        if bytes[idx] == b'<' && starts_tag_name && last_gt.is_some_and(|gt| gt > idx) {
            if let Some(tag_end) = find_tag_end(bytes, idx + 1) {
                let tag_slice = &input[idx..tag_end];
                if tag_has_hidden_attribute(tag_slice) || tag_has_hidden_style(tag_slice) {
                    let name_start = idx + 1;
                    let mut name_end = name_start;
                    while name_end < len
                        && !bytes[name_end].is_ascii_whitespace()
                        && bytes[name_end] != b'>'
                        && bytes[name_end] != b'/'
                    {
                        name_end += 1;
                    }
                    let tag_name = &bytes[name_start..name_end];

                    let remove_end = if is_self_closing_tag(&bytes[idx..tag_end], tag_name) {
                        tag_end
                    } else {
                        find_closing_tag_bytes_nested(bytes, tag_end, tag_name).unwrap_or(tag_end)
                    };

                    let out = output.get_or_insert_with(|| String::with_capacity(len));
                    out.push_str(&input[last..idx]);
                    last = remove_end;
                    idx = remove_end;
                    continue;
                }
            }
        }
        idx += 1;
    }

    if let Some(mut out) = output {
        if last < len {
            out.push_str(&input[last..]);
        }
        Cow::Owned(out)
    } else {
        Cow::Borrowed(input)
    }
}

/// Check if an opening tag string contains the `hidden` attribute.
///
/// Handles: `hidden`, `hidden=""`, `hidden="hidden"`, `hidden="true"`.
/// Does NOT match attributes like `data-hidden` or `aria-hidden`.
///
/// ~keep `pub(crate)`: also called from `tier1::scanner` so the Tier-1 byte
/// ~keep scanner bails on the same hidden-attribute condition this pass strips,
/// ~keep rather than re-implementing the scan a second time.
pub fn tag_has_hidden_attribute(tag: &str) -> bool {
    let bytes = tag.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
        i += 1;
    }

    // ~keep Walk name=value pairs rather than scanning the raw tag text for the word
    // ~keep `hidden`: a plain substring scan also matches inside quoted VALUES, so
    // ~keep `<div title="... hidden from search engines">` read as hidden and the whole
    // ~keep visible element was stripped.
    while i < len {
        while i < len && (bytes[i].is_ascii_whitespace() || bytes[i] == b'/') {
            i += 1;
        }
        if i >= len || bytes[i] == b'>' {
            return false;
        }

        let name_start = i;
        while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b'=' && bytes[i] != b'>' && bytes[i] != b'/' {
            i += 1;
        }
        let name = &bytes[name_start..i];

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < len && bytes[i] == b'=' {
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < len && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                i += 1;
                while i < len && bytes[i] != quote {
                    i += 1;
                }
                i += 1;
            } else {
                while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
                    i += 1;
                }
            }
        }

        if name.eq_ignore_ascii_case(b"hidden") {
            return true;
        }
    }
    false
}

/// Remove `/* ... */` comments from a CSS declaration.
///
/// ~keep A comment before the property name shifts the first `:` so that
/// ~keep `split_once(':')` yields `"/* note */ display"` as the property, which never
/// ~keep matches — silently defeating the hidden-element check and leaking the content.
fn strip_css_comments(declaration: &str) -> Cow<'_, str> {
    if !declaration.contains("/*") {
        return Cow::Borrowed(declaration);
    }
    let mut out = String::with_capacity(declaration.len());
    let mut rest = declaration;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        let Some(end) = rest[start + 2..].find("*/") else {
            rest = "";
            break;
        };
        rest = &rest[start + 2 + end + 2..];
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// Check if an opening tag's inline `style` attribute hides the element via
/// `display: none` or `visibility: hidden`.
///
/// This is a targeted declaration scan, not a full CSS parser: it extracts the raw
/// `style` attribute value, splits it on `;`, and inspects each `property: value`
/// pair. Untrusted input is handled defensively — extra whitespace around `:` and
/// `;`, mixed casing, and a trailing `!important` (with or without a preceding
/// space) are all tolerated.
///
/// ~keep `pub(crate)`: also called from `tier1::scanner` (see
/// ~keep `tag_has_hidden_attribute` above).
pub fn tag_has_hidden_style(tag: &str) -> bool {
    let Some(style_value) = extract_attribute_value(tag, "style") else {
        return false;
    };
    // ~keep CSS cascade: within one declaration block the LAST declaration for a property
    // ~keep wins, so `display:none; display:block` is VISIBLE. Matching with `.any()`
    // ~keep stripped it.
    let mut display_hides = false;
    let mut visibility_hides = false;
    for declaration in style_value.split(';') {
        let cleaned = strip_css_comments(declaration);
        let Some((property, value)) = cleaned.split_once(':') else {
            continue;
        };
        let property = property.trim();
        let value = value.split('!').next().unwrap_or("").trim();
        if property.eq_ignore_ascii_case("display") {
            display_hides = value.eq_ignore_ascii_case("none");
        } else if property.eq_ignore_ascii_case("visibility") {
            visibility_hides = value.eq_ignore_ascii_case("hidden");
        }
    }
    display_hides || visibility_hides
}

/// Check whether a single CSS declaration (`property: value`) hides its element.
fn declaration_hides_element(declaration: &str) -> bool {
    let Some((property, value)) = declaration.split_once(':') else {
        return false;
    };
    let property = property.trim();
    // ~keep `!important` (any casing, with or without a preceding space) is a CSS
    // ~keep priority flag, not part of the value — drop everything from `!` onward.
    let value = value.split('!').next().unwrap_or("").trim();

    (property.eq_ignore_ascii_case("display") && value.eq_ignore_ascii_case("none"))
        || (property.eq_ignore_ascii_case("visibility") && value.eq_ignore_ascii_case("hidden"))
}

/// Extract the value of a named attribute from a raw opening-tag string.
///
/// Walks name=value pairs left to right (skipping the leading tag name) and
/// returns the first case-insensitive match. Handles double- and single-quoted
/// values and bare (unquoted) values. Returns `None` when the attribute is
/// absent or is a boolean attribute with no `=value`.
fn extract_attribute_value<'a>(tag: &'a str, attr_name: &str) -> Option<&'a str> {
    let bytes = tag.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
        i += 1;
    }

    while i < len {
        while i < len && (bytes[i].is_ascii_whitespace() || bytes[i] == b'/') {
            i += 1;
        }
        if i >= len || bytes[i] == b'>' {
            break;
        }

        let name_start = i;
        while i < len && bytes[i] != b'=' && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' && bytes[i] != b'/' {
            i += 1;
        }
        let name = &tag[name_start..i];

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let mut value: Option<&str> = None;
        if i < len && bytes[i] == b'=' {
            i += 1;
            while i < len && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            let (val, next) = scan_attribute_value(bytes, tag, i);
            value = Some(val);
            i = next;
        }

        if name.eq_ignore_ascii_case(attr_name) {
            return value;
        }
    }
    None
}

/// Scan a single attribute value starting at `start`, handling quoted and bare forms.
///
/// Returns the extracted value slice and the byte index immediately following it.
fn scan_attribute_value<'a>(bytes: &[u8], tag: &'a str, start: usize) -> (&'a str, usize) {
    let len = bytes.len();
    if start < len && (bytes[start] == b'"' || bytes[start] == b'\'') {
        let quote = bytes[start];
        let val_start = start + 1;
        let mut end = val_start;
        while end < len && bytes[end] != quote {
            end += 1;
        }
        let value = &tag[val_start..end];
        let next = if end < len { end + 1 } else { end };
        return (value, next);
    }

    let val_start = start;
    let mut end = start;
    while end < len && !bytes[end].is_ascii_whitespace() && bytes[end] != b'>' {
        end += 1;
    }
    (&tag[val_start..end], end)
}

#[cfg(test)]
mod tests {
    use super::{
        find_closing_tag_bytes, find_closing_tag_bytes_nested, normalize_bogus_comment_endings,
        normalize_split_closing_tags, normalize_unclosed_list_items, sanitize_markdown_url, strip_hidden_elements,
    };

    #[test]
    fn normalize_bogus_comment_endings_leaves_well_formed_comment_unchanged() {
        let input = "<p>A</p><!-- foo --><p>B</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_bogus_comment_endings_rewrites_triple_dash_close() {
        let input = "<!-- foo --->";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<!-- foo -->");
    }

    #[test]
    fn normalize_bogus_comment_endings_rewrites_four_dash_close() {
        let input = "<!-- foo ---->";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<!-- foo -->");
    }

    #[test]
    fn normalize_bogus_comment_endings_preserves_content_after_comment() {
        let input = "<h1>One</h1><!-- /// ---><p>Two</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<h1>One</h1><!-- /// --><p>Two</p>");
    }

    #[test]
    fn normalize_bogus_comment_endings_handles_multiple_bogus_comments() {
        let input = "<p>A</p><!-- x ---><p>B</p><!-- y ----><p>C</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), "<p>A</p><!-- x --><p>B</p><!-- y --><p>C</p>");
    }

    #[test]
    fn normalize_bogus_comment_endings_handles_no_comments() {
        let input = "<p>Just a paragraph</p>";
        let result = normalize_bogus_comment_endings(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_bogus_comment_endings_empty_input() {
        let result = normalize_bogus_comment_endings("");
        assert_eq!(result.as_ref(), "");
    }

    #[test]
    fn normalize_split_closing_tags_collapses_newline_before_close_bracket() {
        let input = "<a href=\"#x\">text</a\n>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<a href=\"#x\">text</a>");
    }

    #[test]
    fn normalize_split_closing_tags_collapses_indented_newline_before_close_bracket() {
        let input = "<a href=\"#x\">text</a\n  >";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<a href=\"#x\">text</a>");
    }

    #[test]
    fn normalize_split_closing_tags_leaves_well_formed_closing_tags_unchanged() {
        let input = "<a href=\"#x\">text</a>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_split_closing_tags_handles_multiple_split_closing_tags() {
        let input = "<li><a href=\"#a\">A</a\n  >\n<a href=\"#b\">B</a\n>";
        let result = normalize_split_closing_tags(input);
        assert_eq!(result.as_ref(), "<li><a href=\"#a\">A</a>\n<a href=\"#b\">B</a>");
    }

    #[test]
    fn normalize_split_closing_tags_does_not_collapse_inline_whitespace() {
        let input = "<a href=\"#x\">text</a >";
        let result = normalize_split_closing_tags(input);
        // ~keep A space before > is actually valid HTML and tl handles it fine.
        // ~keep We must not touch it to avoid over-normalising.
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_split_closing_tags_empty_input() {
        let result = normalize_split_closing_tags("");
        assert_eq!(result.as_ref(), "");
    }

    #[test]
    fn sanitize_markdown_url_extracts_scheme_relative_markdown_like_url() {
        let input = "//[p1.zemanta.com/v2/p/ns/45625/PAGE\\_VIEW/](http://p1.zemanta.com/v2/p/ns/45625/PAGE_VIEW/)";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, "http://p1.zemanta.com/v2/p/ns/45625/PAGE_VIEW/");
    }

    #[test]
    fn sanitize_markdown_url_extracts_standard_markdown_like_url() {
        let input = "[label](https://example.com/path?q=1)";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, "https://example.com/path?q=1");
    }

    #[test]
    fn sanitize_markdown_url_leaves_normal_urls_unchanged() {
        let input = "https://example.com/normal";
        let sanitized = sanitize_markdown_url(input);
        assert_eq!(sanitized, input);
    }

    #[test]
    fn normalize_unclosed_list_items_leaves_well_formed_list_unchanged() {
        let input = "<ul><li>A</li><li>B</li></ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn normalize_unclosed_list_items_closes_unclosed_li_before_next_li() {
        let input = "<ul><li>A<li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A</li><li>B</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_closes_chain_of_unclosed_li() {
        let input = "<ul><li>A<li>B<li>C</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A</li><li>B</li><li>C</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_does_not_modify_input_without_list_items() {
        let input = "<p>Hello</p><div>World</div>";
        let result = normalize_unclosed_list_items(input);
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
    }

    #[test]
    fn normalize_unclosed_list_items_handles_nested_list_correctly() {
        let input = "<ul><li>Outer<ul><li>Inner A<li>Inner B</ul><li>Outer B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<ul><li>Outer<ul><li>Inner A</li><li>Inner B</li></ul></li><li>Outer B</li></ul>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_handles_dt_and_dd() {
        let input = "<dl><dt>Term A<dd>Def A<dt>Term B<dd>Def B</dl>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<dl><dt>Term A</dt><dd>Def A</dd><dt>Term B</dt><dd>Def B</dd></dl>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_does_not_touch_content_in_pre() {
        let input = "<ul><li>A<pre><li>not-a-list-item</pre><li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(
            result.as_ref(),
            "<ul><li>A<pre><li>not-a-list-item</pre></li><li>B</li></ul>"
        );
    }

    #[test]
    fn normalize_unclosed_list_items_skips_html_comments() {
        let input = "<ul><li>A<!-- <li>comment --><li>B</ul>";
        let result = normalize_unclosed_list_items(input);
        assert_eq!(result.as_ref(), "<ul><li>A<!-- <li>comment --></li><li>B</li></ul>");
    }

    #[test]
    fn normalize_unclosed_list_items_empty_input() {
        let result = normalize_unclosed_list_items("");
        assert_eq!(result.as_ref(), "");
    }

    #[test]
    fn strip_hidden_elements_removes_display_none_element() {
        let input = r#"<p>visible</p><div style="display:none">secret</div><p>also visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p><p>also visible</p>");
    }

    #[test]
    fn strip_hidden_elements_removes_visibility_hidden_element() {
        let input = r#"<p>visible</p><span style="visibility:hidden">secret</span><p>also visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p><p>also visible</p>");
    }

    #[test]
    fn strip_hidden_elements_tolerates_whitespace_around_declaration() {
        let input = r#"<div style="display : none">secret</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_tolerates_mixed_case_declaration() {
        let input = r#"<div style="Display:NONE">secret</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_tolerates_important_flag() {
        let input = r#"<div style="display:none !important">secret</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_matches_declaration_among_others() {
        let input = r#"<div style="color:red; display:none; margin:0">secret</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_leaves_visible_style_untouched() {
        let input = r#"<div style="color:red; display:block">visible</div>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), input);
    }

    #[test]
    fn strip_hidden_elements_removes_nested_content_inside_hidden_parent() {
        let input = r#"<div style="display:none"><p>secret</p><span>also secret</span></div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_removes_parent_that_nests_the_same_tag_name() {
        let input = r#"<p>A<div style="display:none">S1<div>S2</div>LEAKED</div>B</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>AB</p>");
    }

    #[test]
    fn strip_hidden_elements_removes_deeply_nested_same_tag_subtree() {
        let input = r#"<div style="display:none">L0<div>L1<div>L2</div>T2</div>T1</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_matches_closing_tag_case_insensitively() {
        let input = r#"<DIV style="display:none">x<div>y</div>z</DIV><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_does_not_match_a_tag_with_a_longer_name() {
        let input = r#"<div style="display:none">a<divider>b</divider>c</div><p>visible</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>");
    }

    #[test]
    fn strip_hidden_elements_drops_only_the_open_tag_when_never_closed() {
        let input = r#"<p>visible</p><div style="display:none">dangling"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>visible</p>dangling");
    }

    #[test]
    fn find_closing_tag_bytes_nested_skips_inner_same_name_element() {
        let html = b"<div>a<div>b</div>c</div>tail";
        let start = "<div>".len();
        let end = find_closing_tag_bytes_nested(html, start, b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_self_closing_inner_tag() {
        let html = b"<div>a<div/>b</div>tail";
        let start = "<div>".len();
        let end = find_closing_tag_bytes_nested(html, start, b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_returns_none_for_unbalanced_input() {
        let html = b"<div>a<div>b</div>";
        assert_eq!(find_closing_tag_bytes_nested(html, "<div>".len(), b"div"), None);
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_close_tag_inside_a_comment() {
        let html = b"<div>a<!-- </div> -->b</div>tail";
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_open_tag_inside_a_comment() {
        let html = b"<div>a<!-- <div> -->b</div>tail";
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_close_tag_inside_a_quoted_attribute() {
        let html = br#"<div>a<span title="</div>">x</span>b</div>tail"#;
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_close_tag_inside_a_raw_text_body() {
        let html = br#"<div>a<script>var s = "</div>";</script>b</div>tail"#;
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_ignores_close_tag_inside_a_cdata_section() {
        let html = b"<div>a<![CDATA[ </div> ]]>b</div>tail";
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn find_closing_tag_bytes_nested_does_not_skip_over_a_literal_less_than_in_text() {
        let html = b"<div>a < b</div>tail";
        let end = find_closing_tag_bytes_nested(html, "<div>".len(), b"div").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }

    #[test]
    fn strip_hidden_elements_ignores_a_close_tag_written_inside_a_comment() {
        let input = r#"<p>A</p><div style="display:none">SECRET<!-- </div> -->MORE</div><p>B</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), "<p>A</p><p>B</p>");
    }

    #[test]
    fn strip_hidden_elements_keeps_sibling_when_a_comment_holds_an_unbalanced_open_tag() {
        let input = r#"<div id="wrap"><div style="display:none">S<!-- <div> --></div><p>VISIBLE</p></div><p>after</p>"#;
        let result = strip_hidden_elements(input);
        assert_eq!(result.as_ref(), r#"<div id="wrap"><p>VISIBLE</p></div><p>after</p>"#);
    }

    #[test]
    fn find_closing_tag_bytes_stays_first_match_for_raw_text_elements() {
        let html = br#"<script>var s = "<script>";</script>tail"#;
        let start = "<script>".len();
        let end = find_closing_tag_bytes(html, start, b"script").expect("closing tag not found");
        assert_eq!(&html[end..], b"tail");
    }
}
