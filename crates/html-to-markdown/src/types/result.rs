//! The primary result type for HTML conversion and extraction.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::document::DocumentStructure;
use super::tables::TableData;
use super::warnings::ProcessingWarning;

/// The primary result of HTML conversion and extraction.
///
/// Contains the converted text output, optional structured document tree,
/// metadata, extracted tables, images, and processing warnings.
///
/// # Example
///
/// ```text
/// use html_to_markdown_rs::{convert, ConversionOptions};
///
/// let result = convert("<h1>Hello</h1><p>World</p>", None)?;
/// assert!(result.content.is_some());
/// assert!(result.warnings.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversionResult {
    /// Converted text output in the selected format: Markdown, Djot, or plain text.
    pub content: Option<String>,

    /// Structured document tree with semantic elements.
    ///
    /// Populated when `ConversionOptions::include_document_structure` is `true`. `None`
    /// otherwise (the default), which avoids the overhead of building the tree.
    ///
    /// When present, the tree mirrors the converted document: headings open
    /// [`crate::types::document::NodeContent::Group`] sections, paragraphs and list items carry
    /// inline [`crate::types::document::TextAnnotation`]s, and tables reference the same
    /// [`crate::types::tables::TableGrid`] data exposed in [`Self::tables`].
    ///
    /// Note: this field is independent of the `metadata` feature flag. Document structure
    /// collection is always available at runtime; it is gated only by the runtime option, not
    /// by a compile-time feature.
    pub document: Option<DocumentStructure>,

    /// Extracted HTML metadata (title, OG, links, images, structured data).
    #[cfg(feature = "metadata")]
    pub metadata: crate::metadata::HtmlMetadata,

    /// Extracted tables with structured cell data and markdown representation.
    ///
    /// Table data is collected by the same pass that builds [`Self::document`], so it is
    /// populated only when `ConversionOptions::include_document_structure` is `true`. With the
    /// default options this is an empty vec even for input that contains tables — the tables
    /// still appear in [`Self::content`] as rendered Markdown.
    pub tables: Vec<TableData>,

    /// Extracted inline images from data URIs and SVGs.
    ///
    /// Populated when the `inline-images` feature is enabled and
    /// `extract_images` is `true`. Bindings may expose a simplified image
    /// representation or omit this Rust-only payload depending on backend
    /// support for binary image data.
    #[cfg(feature = "inline-images")]
    #[cfg_attr(feature = "serde", serde(skip))]
    #[cfg_attr(alef, alef(skip))]
    pub images: Vec<crate::inline_images::InlineImage>,

    /// Non-fatal processing warnings.
    pub warnings: Vec<ProcessingWarning>,
}
