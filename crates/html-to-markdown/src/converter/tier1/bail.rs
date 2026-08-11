//! Bail reasons emitted by the Tier-1 scanner.
//!
//! When the scanner encounters a condition it cannot handle correctly it returns
//! one of these variants. The dispatcher in `convert_api.rs::convert_inner` catches the
//! error, logs it at `tracing::warn!` (a fallback was taken), and falls back to the
//! Tier-2 path with the original (pre-prescan) input.

use std::fmt;

/// Reasons the Tier-1 scanner may bail out and hand off to Tier-2.
#[derive(Debug, Clone)]
pub enum BailReason {
    /// The classifier decided Tier-2 is required for this input / option set.
    Classifier,

    /// An open-tag stack mismatch was detected mid-stream.
    DepthMismatch {
        /// Tag name where the mismatch was detected.
        tag: String,
        /// Depth expected by the scanner's stack.
        expected: u8,
        /// Depth seen in the source.
        actual: u8,
    },

    /// Reached end-of-file with one or more unclosed block elements.
    EofWithOpenBlock {
        /// Number of unclosed block elements at EOF.
        open_count: usize,
    },

    /// A literal `<` (not a valid tag open) was encountered in the stream.
    LiteralLt {
        /// Byte offset of the `<` in the input.
        offset: usize,
    },

    /// A CDATA section was encountered.
    Cdata {
        /// Byte offset of `<![CDATA[` in the input.
        offset: usize,
    },

    /// An unknown custom element (tag name containing `-`) was encountered.
    UnknownCustomElement {
        /// The element name.
        name: Box<str>,
        /// Byte offset of the `<` in the input.
        offset: usize,
    },

    /// Two `<script>`/`<style>` (raw-text-ignored) elements were found directly
    /// adjacent, with no separating whitespace.  Tier-2's script/style-stripping
    /// preprocessing pass collapses such pairs into a single whitespace-only DOM
    /// text node whose downstream handling produces a byte pattern Tier-1 does
    /// not replicate; bail so Tier-2 (authoritative) handles it.
    AdjacentRawTextTags {
        /// Byte offset of the second element's `<` in the input.
        offset: usize,
    },

    // ~keep ── Table-specific bail reasons ───────────────────────────────────────────
    /// A `<td>` or `<th>` had a `rowspan` or `colspan` attribute with a value
    /// other than 1 (absent attribute counts as 1).
    TableRowspanColspan,

    /// A block-level element was opened inside a `<td>` or `<th>` (e.g.
    /// `<td><p>text</p></td>`).  Tier-1 only supports inline cell content.
    TableBlockChildInCell,

    /// A nested `<table>` was opened while a table is already being assembled
    /// (i.e. `table_stack` is non-empty).
    TableNestedTable,

    /// A `<caption>` element was encountered inside a table.
    TableCaption,

    /// Table sections appear in an unsupported order, e.g. `<tbody>` after
    /// `<tfoot>` close, or `<thead>` after any section that already closed.
    TableSectionOrder,

    /// Open-tag nesting reached the effective depth limit
    /// (`crate::converter::main_helpers::effective_max_depth`).
    ///
    /// The scanner's `state.stack` is an explicit `Vec`, not native recursion, so
    /// it has no stack-overflow risk of its own — but Tier-2's recursive
    /// `walk_node` silently truncates (skips deeper nodes and their content)
    /// once `depth >= effective_max_depth`. Continuing the scan past that same
    /// depth would produce the *untruncated* output, diverging from Tier-2's
    /// authoritative truncated output. Bail so Tier-2 (which truncates) wins.
    DepthLimitExceeded {
        /// Nesting depth (open-tag count) at which the limit was reached.
        depth: usize,
        /// The effective limit that was exceeded.
        max_depth: usize,
    },

    /// A named HTML entity (e.g. `&mdash;`, `&laquo;`) was encountered that is
    /// not in Tier-1's 45-entry decode table, or a numeric character reference
    /// was malformed / mapped to an invalid Unicode code point.
    ///
    /// Tier-1 would pass the entity through verbatim, but Tier-2 decodes it to
    /// the correct character, so the outputs would diverge.  Bail so the
    /// dispatcher falls back to Tier-2.
    UnknownEntity {
        /// The entity name between `&` and `;` (e.g. `"mdash"`, `"#x2014"`).
        name: Box<str>,
        /// Byte offset in the HTML input where the `&` was found.
        offset: usize,
    },

    /// An opening tag carries the `hidden` attribute, or an inline `style`
    /// declaration that hides the element (`display: none` / `visibility:
    /// hidden`).
    ///
    /// `converter::utility::preprocessing::strip_hidden_elements` (outside
    /// tier1/) removes such elements — tag and all descendant content —
    /// unconditionally before Tier-2 ever parses the document. Tier-1 has no
    /// equivalent pass and would otherwise emit the hidden element's content
    /// verbatim. Bail so Tier-2 (which already strips it) is authoritative.
    HiddenElement {
        /// Byte offset of the element's `<` in the input.
        offset: usize,
    },

    /// A list (`<ul>`/`<ol>`) was opened while already nested inside another
    /// list, where either the new list or an ancestor list is `<ol>`.
    ///
    /// A nested list's indent must equal the cumulative width of every
    /// ancestor marker (`"- "` = 2, `"1. "` = 3, `"10. "` = 4, ...). Tier-1's
    /// `push_list_item_indent` hardcodes a uniform 2-space-per-depth scheme,
    /// which only holds when every list in the ancestor chain is unordered.
    /// Bail so Tier-2 (which computes cumulative marker widths) is authoritative.
    ListNestedOrdered,
}

impl fmt::Display for BailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classifier => write!(f, "classifier forced tier-2"),
            Self::DepthMismatch { tag, expected, actual } => {
                write!(
                    f,
                    "depth mismatch for </{tag}>: expected {expected} open(s), got {actual}"
                )
            }
            Self::EofWithOpenBlock { open_count } => {
                write!(f, "EOF with {open_count} unclosed block element(s)")
            }
            Self::LiteralLt { offset } => {
                write!(f, "literal '<' at byte offset {offset}")
            }
            Self::Cdata { offset } => {
                write!(f, "CDATA section at byte offset {offset}")
            }
            Self::UnknownCustomElement { name, offset } => {
                write!(f, "unknown custom element <{name}> at byte offset {offset}")
            }
            Self::AdjacentRawTextTags { offset } => {
                write!(
                    f,
                    "adjacent <script>/<style> tags with no separating whitespace at byte offset {offset}"
                )
            }
            Self::TableRowspanColspan => {
                write!(f, "table cell has rowspan or colspan != 1")
            }
            Self::TableBlockChildInCell => {
                write!(f, "block-level element inside table cell")
            }
            Self::TableNestedTable => {
                write!(f, "nested <table> inside a table cell")
            }
            Self::TableCaption => {
                write!(f, "<caption> element in table")
            }
            Self::TableSectionOrder => {
                write!(f, "table sections in unsupported order")
            }
            Self::UnknownEntity { name, offset } => {
                write!(f, "unknown HTML entity &{name}; at byte offset {offset}")
            }
            Self::DepthLimitExceeded { depth, max_depth } => {
                write!(
                    f,
                    "open-tag nesting depth {depth} reached the effective limit of {max_depth}"
                )
            }
            Self::HiddenElement { offset } => {
                write!(f, "hidden element (hidden attribute or style) at byte offset {offset}")
            }
            Self::ListNestedOrdered => {
                write!(
                    f,
                    "nested list with an ordered ancestor or ordered self (cumulative indent width)"
                )
            }
        }
    }
}
