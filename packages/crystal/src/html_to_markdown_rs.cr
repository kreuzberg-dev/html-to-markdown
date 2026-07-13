require "json"

# Low-level binding to the generated C FFI layer (html_to_markdown.h).
#
# Every non-scalar value crosses the C ABI as a NUL-terminated JSON string
# (`LibC::Char*`); scalars pass by value. Strings returned by the library are
# owned by Rust and must be released with `htm_free_string`.
#
# Link against the FFI shared library. The library must be installed to a
# standard path, or you can pass --link-flags at build time:
#   crystal build ... --link-flags="-L/path/to/lib -Wl,-rpath,/path/to/lib"
@[Link(ldflags: "-lhtml_to_markdown_ffi")]
lib LibHtm
  fun free_string = htm_free_string(ptr : LibC::Char*) : Void
  fun last_error_code = htm_last_error_code() : Int32
  fun last_error_context = htm_last_error_context() : LibC::Char*

  struct ConversionOptions
    _data : Void*
  end
  struct ConversionResult
    _data : Void*
  end
  struct NodeContext
    _data : Void*
  end
  struct NodeType
    _data : Void*
  end
  struct PreprocessingOptions
    _data : Void*
  end
  struct VisitResult
    _data : Void*
  end
  fun conversion_options_from_json = htm_conversion_options_from_json(json : LibC::Char*) : ConversionOptions*
  fun conversion_options_to_json = htm_conversion_options_to_json(ptr : ConversionOptions*) : LibC::Char*
  fun conversion_options_free = htm_conversion_options_free(ptr : ConversionOptions*)
  fun conversion_result_from_json = htm_conversion_result_from_json(json : LibC::Char*) : ConversionResult*
  fun conversion_result_to_json = htm_conversion_result_to_json(ptr : ConversionResult*) : LibC::Char*
  fun conversion_result_free = htm_conversion_result_free(ptr : ConversionResult*)
  fun node_context_from_json = htm_node_context_from_json(json : LibC::Char*) : NodeContext*
  fun node_context_to_json = htm_node_context_to_json(ptr : NodeContext*) : LibC::Char*
  fun node_context_free = htm_node_context_free(ptr : NodeContext*)
  fun node_type_from_json = htm_node_type_from_json(json : LibC::Char*) : NodeType*
  fun node_type_to_json = htm_node_type_to_json(ptr : NodeType*) : LibC::Char*
  fun node_type_free = htm_node_type_free(ptr : NodeType*)
  fun preprocessing_options_from_json = htm_preprocessing_options_from_json(json : LibC::Char*) : PreprocessingOptions*
  fun preprocessing_options_to_json = htm_preprocessing_options_to_json(ptr : PreprocessingOptions*) : LibC::Char*
  fun preprocessing_options_free = htm_preprocessing_options_free(ptr : PreprocessingOptions*)
  fun visit_result_from_json = htm_visit_result_from_json(json : LibC::Char*) : VisitResult*
  fun visit_result_to_json = htm_visit_result_to_json(ptr : VisitResult*) : LibC::Char*
  fun visit_result_free = htm_visit_result_free(ptr : VisitResult*)

  # Convert HTML to Markdown, Djot, or plain text.
  fun convert = htm_convert(html : LibC::Char*, options : ConversionOptions*) : ConversionResult*
  fun html_visitor_visit_text = htm_html_visitor_visit_text(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_element_start = htm_html_visitor_visit_element_start(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_element_end = htm_html_visitor_visit_element_end(handle : Void*, ctx : NodeContext*, output : LibC::Char*) : VisitResult*
  fun html_visitor_visit_link = htm_html_visitor_visit_link(handle : Void*, ctx : NodeContext*, href : LibC::Char*, text : LibC::Char*, title : LibC::Char*) : VisitResult*
  fun html_visitor_visit_image = htm_html_visitor_visit_image(handle : Void*, ctx : NodeContext*, src : LibC::Char*, alt : LibC::Char*, title : LibC::Char*) : VisitResult*
  fun html_visitor_visit_heading = htm_html_visitor_visit_heading(handle : Void*, ctx : NodeContext*, level : UInt32, text : LibC::Char*, id : LibC::Char*) : VisitResult*
  fun html_visitor_visit_code_block = htm_html_visitor_visit_code_block(handle : Void*, ctx : NodeContext*, lang : LibC::Char*, code : LibC::Char*) : VisitResult*
  fun html_visitor_visit_code_inline = htm_html_visitor_visit_code_inline(handle : Void*, ctx : NodeContext*, code : LibC::Char*) : VisitResult*
  fun html_visitor_visit_list_item = htm_html_visitor_visit_list_item(handle : Void*, ctx : NodeContext*, ordered : Bool, marker : LibC::Char*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_list_start = htm_html_visitor_visit_list_start(handle : Void*, ctx : NodeContext*, ordered : Bool) : VisitResult*
  fun html_visitor_visit_list_end = htm_html_visitor_visit_list_end(handle : Void*, ctx : NodeContext*, ordered : Bool, output : LibC::Char*) : VisitResult*
  fun html_visitor_visit_table_start = htm_html_visitor_visit_table_start(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_table_row = htm_html_visitor_visit_table_row(handle : Void*, ctx : NodeContext*, cells : LibC::Char*, is_header : Bool) : VisitResult*
  fun html_visitor_visit_table_end = htm_html_visitor_visit_table_end(handle : Void*, ctx : NodeContext*, output : LibC::Char*) : VisitResult*
  fun html_visitor_visit_blockquote = htm_html_visitor_visit_blockquote(handle : Void*, ctx : NodeContext*, content : LibC::Char*, depth : LibC::SizeT) : VisitResult*
  fun html_visitor_visit_strong = htm_html_visitor_visit_strong(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_emphasis = htm_html_visitor_visit_emphasis(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_strikethrough = htm_html_visitor_visit_strikethrough(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_underline = htm_html_visitor_visit_underline(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_subscript = htm_html_visitor_visit_subscript(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_superscript = htm_html_visitor_visit_superscript(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_mark = htm_html_visitor_visit_mark(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_line_break = htm_html_visitor_visit_line_break(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_horizontal_rule = htm_html_visitor_visit_horizontal_rule(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_custom_element = htm_html_visitor_visit_custom_element(handle : Void*, ctx : NodeContext*, tag_name : LibC::Char*, html : LibC::Char*) : VisitResult*
  fun html_visitor_visit_definition_list_start = htm_html_visitor_visit_definition_list_start(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_definition_term = htm_html_visitor_visit_definition_term(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_definition_description = htm_html_visitor_visit_definition_description(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_definition_list_end = htm_html_visitor_visit_definition_list_end(handle : Void*, ctx : NodeContext*, output : LibC::Char*) : VisitResult*
  fun html_visitor_visit_form = htm_html_visitor_visit_form(handle : Void*, ctx : NodeContext*, action : LibC::Char*, method : LibC::Char*) : VisitResult*
  fun html_visitor_visit_input = htm_html_visitor_visit_input(handle : Void*, ctx : NodeContext*, input_type : LibC::Char*, name : LibC::Char*, value : LibC::Char*) : VisitResult*
  fun html_visitor_visit_button = htm_html_visitor_visit_button(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_audio = htm_html_visitor_visit_audio(handle : Void*, ctx : NodeContext*, src : LibC::Char*) : VisitResult*
  fun html_visitor_visit_video = htm_html_visitor_visit_video(handle : Void*, ctx : NodeContext*, src : LibC::Char*) : VisitResult*
  fun html_visitor_visit_iframe = htm_html_visitor_visit_iframe(handle : Void*, ctx : NodeContext*, src : LibC::Char*) : VisitResult*
  fun html_visitor_visit_details = htm_html_visitor_visit_details(handle : Void*, ctx : NodeContext*, open : Bool) : VisitResult*
  fun html_visitor_visit_summary = htm_html_visitor_visit_summary(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_figure_start = htm_html_visitor_visit_figure_start(handle : Void*, ctx : NodeContext*) : VisitResult*
  fun html_visitor_visit_figcaption = htm_html_visitor_visit_figcaption(handle : Void*, ctx : NodeContext*, text : LibC::Char*) : VisitResult*
  fun html_visitor_visit_figure_end = htm_html_visitor_visit_figure_end(handle : Void*, ctx : NodeContext*, output : LibC::Char*) : VisitResult*
  fun html_visitor_free = htm_html_visitor_free(handle : Void*) : Void
  fun visitor_handle_free = htm_visitor_handle_free(handle : Void*) : Void
end

# html-to-markdown-rs — Crystal bindings generated by alef.
#
# Ruby-style API over the Rust core: snake_case methods, PascalCase types,
# Rust-like generic containers (`Array(T)`, `Hash(K, V)`), and fiber/`Channel`
# based concurrency for async and streaming methods.
module HtmlToMarkdown
  VERSION = "3.8.3"

  # Document-level metadata extracted from `<head>` and top-level elements.
  #
  # Contains all metadata typically used by search engines, social media platforms,
  # and browsers for document indexing and presentation.
  class DocumentMetadata
    include JSON::Serializable
    # Document title from `<title>` tag
    getter title : String?
    # Document description from `<meta name="description">` tag
    getter description : String?
    # Document keywords from `<meta name="keywords">` tag, split on commas
    getter keywords : Array(String) = [] of String
    # Document author from `<meta name="author">` tag
    getter author : String?
    # Canonical URL from `<link rel="canonical">` tag
    getter canonical_url : String?
    # Base URL from `<base href="">` tag for resolving relative URLs
    getter base_href : String?
    # Document language from `lang` attribute
    getter language : String?
    # Document text direction from `dir` attribute
    getter text_direction : TextDirection?
    # Open Graph metadata (og:* properties) for social media
    # Keys like "title", "description", "image", "url", etc.
    getter open_graph : Hash(String, String) = {} of String => String
    # Twitter Card metadata (twitter:* properties)
    # Keys like "card", "site", "creator", "title", "description", "image", etc.
    getter twitter_card : Hash(String, String) = {} of String => String
    # Additional meta tags not covered by specific fields
    # Keys are meta name/property attributes, values are content
    getter meta_tags : Hash(String, String) = {} of String => String
  end

  # Header element metadata with hierarchy tracking.
  #
  # Captures heading elements (h1-h6) with their text content, identifiers,
  # and position in the document structure.
  class HeaderMetadata
    include JSON::Serializable
    # Header level: 1 (h1) through 6 (h6)
    getter level : UInt8 = 0
    # Normalized text content of the header
    getter text : String = ""
    # HTML id attribute if present
    getter id : String?
    # Document tree depth at the header element
    getter depth : UInt64 = 0
    # Byte offset in original HTML document
    getter html_offset : UInt64 = 0
  end

  # Hyperlink metadata with categorization and attributes.
  #
  # Represents `<a>` elements with parsed href values, text content, and link type classification.
  class LinkMetadata
    include JSON::Serializable
    # The href URL value
    getter href : String = ""
    # Link text content (normalized, concatenated if mixed with elements)
    getter text : String = ""
    # Optional title attribute (often shown as tooltip)
    getter title : String?
    # Link type classification
    getter link_type : LinkType = LinkType::Anchor
    # Rel attribute values (e.g., "nofollow", "stylesheet", "canonical")
    getter rel : Array(String) = [] of String
    # Additional HTML attributes
    getter attributes : Hash(String, String) = {} of String => String
  end

  # Image metadata with source and dimensions.
  #
  # Captures `<img>` elements and inline `<svg>` elements with metadata
  # for image analysis and optimization.
  class ImageMetadata
    include JSON::Serializable
    # Image source (URL, data URI, or SVG content identifier)
    getter src : String = ""
    # Alternative text from alt attribute (for accessibility)
    getter alt : String?
    # Title attribute (often shown as tooltip)
    getter title : String?
    # Image dimensions in pixels, if available.
    getter dimensions : ImageDimensions?
    # Image type classification
    getter image_type : ImageType = ImageType::DataUri
    # Additional HTML attributes
    getter attributes : Hash(String, String) = {} of String => String
  end

  # Structured data block (JSON-LD, Microdata, or `RDFa`).
  #
  # Represents machine-readable structured data found in the document.
  # JSON-LD blocks are collected as raw JSON strings for flexibility.
  class StructuredData
    include JSON::Serializable
    # Type of structured data (JSON-LD, Microdata, `RDFa`)
    getter data_type : StructuredDataType = StructuredDataType::JsonLd
    # Raw JSON string (for JSON-LD) or serialized representation
    getter raw_json : String = ""
    # Schema type if detectable (e.g., "Article", "Event", "Product")
    getter schema_type : String?
  end

  # Comprehensive metadata extraction result from HTML document.
  #
  # Contains all extracted metadata types in a single structure,
  # suitable for serialization and transmission across language boundaries.
  class HtmlMetadata
    include JSON::Serializable
    # Document-level metadata (title, description, canonical, etc.)
    getter document : DocumentMetadata = DocumentMetadata.from_json("{}")
    # Extracted header elements with hierarchy
    getter headers : Array(HeaderMetadata) = [] of HeaderMetadata
    # Extracted hyperlinks with type classification
    getter links : Array(LinkMetadata) = [] of LinkMetadata
    # Extracted images with source and dimensions
    getter images : Array(ImageMetadata) = [] of ImageMetadata
    # Extracted structured data blocks
    getter structured_data : Array(StructuredData) = [] of StructuredData
  end

  # Main conversion options for HTML to Markdown conversion.
  #
  # Use [`ConversionOptions::builder()`] to construct, or [`Default::default()`] for defaults.
  class ConversionOptions
    include JSON::Serializable
    # Heading style to use in Markdown output (ATX `#` or Setext underline).
    getter heading_style : HeadingStyle = HeadingStyle::Atx
    # How to indent nested list items (spaces or tab).
    getter list_indent_type : ListIndentType = ListIndentType::Spaces
    # Number of spaces (or tabs) to use for each level of list indentation.
    getter list_indent_width : UInt64 = 2
    # Bullet character(s) to use for unordered list items (e.g. `"-"`, `"*"`).
    getter bullets : String = "-*+"
    # Character used for bold/italic emphasis markers (`*` or `_`).
    getter strong_em_symbol : String = "*"
    # Escape `*` characters in plain text to avoid unintended bold/italic.
    getter escape_asterisks : Bool = false
    # Escape `_` characters in plain text to avoid unintended bold/italic.
    getter escape_underscores : Bool = false
    # Escape miscellaneous Markdown metacharacters (`[]()#` etc.) in plain text.
    getter escape_misc : Bool = false
    # Escape ASCII characters that have special meaning in certain Markdown dialects.
    getter escape_ascii : Bool = false
    # Default language annotation for fenced code blocks that have no language hint.
    getter code_language : String = ""
    # Automatically convert bare URLs into Markdown autolinks.
    getter autolinks : Bool = true
    # Emit a default title when no `<title>` tag is present.
    getter default_title : Bool = false
    # Render `<br>` elements inside table cells as literal line breaks.
    getter br_in_tables : Bool = false
    # Emit tables without column padding (compact GFM format).
    #
    # When `true`, column widths are not computed and cells are emitted with
    # no trailing spaces. Separator rows use exactly `---` per column.
    # Produces token-efficient output suitable for RAG / LLM contexts.
    #
    # Default `false` (aligned padding preserved).
    getter compact_tables : Bool = false
    # Style used for `<mark>` / highlighted text (e.g. `==text==`).
    getter highlight_style : HighlightStyle = HighlightStyle::DoubleEqual
    # Populate `result.metadata` with `<head>` / `<meta>` extraction
    # (title, description, Open Graph, Twitter Card, JSON-LD, …).
    #
    # Default `true`. Disabling skips the metadata pass only — table
    # extraction into `result.tables` runs unconditionally.
    getter extract_metadata : Bool = true
    # Controls how whitespace sequences are normalised in the converted output.
    #
    # - [`WhitespaceMode::Normalized`] (default) — collapses consecutive whitespace characters
    #   (spaces, tabs, newlines) to a single space, matching browser rendering behaviour.
    # - [`WhitespaceMode::Strict`] — preserves all whitespace exactly as it appears in the
    #   source HTML, including runs of spaces and embedded newlines.
    #
    # Choose `Strict` only when the source HTML uses deliberate whitespace (e.g. pre-formatted
    # content outside `<pre>` tags). For most documents `Normalized` produces cleaner output.
    getter whitespace_mode : WhitespaceMode = WhitespaceMode::Normalized
    # Strip all newlines from the output, producing a single-line result.
    getter strip_newlines : Bool = false
    # Wrap long lines at [`wrap_width`](Self::wrap_width) characters.
    getter wrap : Bool = false
    # Maximum output line width in characters when [`wrap`](Self::wrap) is `true` (default `80`).
    #
    # Lines are broken at word boundaries so that no line exceeds this length. A value of `0`
    # is treated as "no limit" — equivalent to leaving [`wrap`](Self::wrap) disabled. Has no
    # effect when `wrap` is `false`.
    getter wrap_width : UInt64 = 80
    # Treat the entire document as inline content (no block-level wrappers).
    getter convert_as_inline : Bool = false
    # Markdown notation for subscript text (e.g. `"~"`).
    getter sub_symbol : String = ""
    # Markdown notation for superscript text (e.g. `"^"`).
    getter sup_symbol : String = ""
    # How to encode hard line breaks (`<br>`) in Markdown.
    getter newline_style : NewlineStyle = NewlineStyle::Spaces
    # Style used for fenced code blocks (backticks or tilde).
    getter code_block_style : CodeBlockStyle = CodeBlockStyle::Backticks
    # HTML tag names whose `<img>` children are kept inline instead of block.
    getter keep_inline_images_in : Array(String) = [] of String
    # Options for the HTML pre-processing pass applied before conversion begins.
    #
    # Pre-processing runs before the HTML is handed to the converter and can perform operations
    # such as unwrapping redundant wrapper elements, removing tracking pixels, and normalising
    # vendor-specific markup. See [`PreprocessingOptions`] for the full set of knobs.
    #
    # Defaults to [`PreprocessingOptions::default()`], which enables the standard cleaning
    # passes. Set individual fields on [`PreprocessingOptions`] (or construct via
    # [`ConversionOptions::builder`]) to opt in or out of specific passes.
    getter preprocessing : PreprocessingOptions = PreprocessingOptions.from_json("{}")
    # Expected character encoding of the input HTML (default `"utf-8"`).
    getter encoding : String = "utf-8"
    # Emit debug information during conversion.
    getter debug : Bool = false
    # HTML tag names whose content is stripped from the output entirely.
    getter strip_tags : Array(String) = [] of String
    # HTML tag names that are preserved verbatim in the output.
    getter preserve_tags : Array(String) = [] of String
    # Skip conversion of `<img>` elements (omit images from output).
    getter skip_images : Bool = false
    # URL encoding strategy for link and image destinations.
    #
    # Controls how special characters in URL destinations are escaped:
    # - [`UrlEscapeStyle::Angle`] (default) — wraps the destination in angle brackets when it
    #   contains spaces or newlines. Some parsers misinterpret `>` inside such a destination.
    # - [`UrlEscapeStyle::Percent`] — percent-encodes every character that is not an RFC 3986
    #   unreserved character or `/`, producing a destination that all Markdown parsers handle
    #   correctly even when the URL contains `<`, `>`, spaces, or parentheses.
    getter url_escape_style : UrlEscapeStyle = UrlEscapeStyle::Angle
    # Link rendering style (inline or reference).
    getter link_style : LinkStyle = LinkStyle::Inline
    # Target output format (Markdown, plain text, etc.).
    getter output_format : OutputFormat = OutputFormat::Markdown
    # Include structured document tree in result.
    getter include_document_structure : Bool = false
    # Extract inline images from data URIs and SVGs.
    getter extract_images : Bool = false
    # Maximum decoded image size in bytes (default 5MB).
    getter max_image_size : UInt64 = 5242880
    # Capture SVG elements as images.
    getter capture_svg : Bool = false
    # Infer image dimensions from data.
    getter infer_dimensions : Bool = true
    # Maximum DOM traversal depth.
    #
    # `None` uses the library's internal native-stack safety limit. Explicit
    # values above that safety limit are clamped to prevent process-aborting
    # stack overflows on pathologically deep DOM trees.
    getter max_depth : UInt64?
    # CSS selectors for elements to exclude entirely (element + all content).
    #
    # Unlike `strip_tags` (which removes the tag wrapper but keeps children),
    # excluded elements and all their descendants are dropped from the output.
    # Supports any CSS selector that `tl` supports: tag names, `.class`,
    # `#id`, `[attribute]`, etc.
    #
    # Invalid selectors are silently skipped at conversion time.
    #
    # Example: `vec![".cookie-banner".into(), "#ad-container".into(), "[role='complementary']".into()]`
    getter exclude_selectors : Array(String) = [] of String
    # Which conversion tier to use.
    #
    # - [`TierStrategy::Auto`] (default) — automatically choose the best path.
    # - [`TierStrategy::Tier2`] — always use the Tier-2 DOM-walk path.
    # - `TierStrategy::Tier1` — always attempt Tier-1 (testkit only).
    getter tier_strategy : TierStrategy = TierStrategy::Auto
    # Optional visitor for custom traversal logic.
    #
    # When set, the visitor's callbacks are invoked for matching HTML elements
    # during conversion, allowing custom output, skipping, or HTML preservation.
    # See `HtmlVisitor`.
    @[JSON::Field(ignore: true)]
    getter visitor : VisitorHandle?
  end

  # Partial update for `ConversionOptions`.
  #
  # Uses `Option<T>` fields for selective updates. Bindings use this to construct
  # options from language-native types. Prefer [`ConversionOptionsBuilder`] for Rust code.
  class ConversionOptionsUpdate
    include JSON::Serializable
    # Optional override for [`ConversionOptions::heading_style`].
    getter heading_style : HeadingStyle?
    # Optional override for [`ConversionOptions::list_indent_type`].
    getter list_indent_type : ListIndentType?
    # Optional override for [`ConversionOptions::list_indent_width`].
    getter list_indent_width : UInt64?
    # Optional override for [`ConversionOptions::bullets`].
    getter bullets : String?
    # Optional override for [`ConversionOptions::strong_em_symbol`].
    getter strong_em_symbol : String?
    # Optional override for [`ConversionOptions::escape_asterisks`].
    getter escape_asterisks : Bool?
    # Optional override for [`ConversionOptions::escape_underscores`].
    getter escape_underscores : Bool?
    # Optional override for [`ConversionOptions::escape_misc`].
    getter escape_misc : Bool?
    # Optional override for [`ConversionOptions::escape_ascii`].
    getter escape_ascii : Bool?
    # Optional override for [`ConversionOptions::code_language`].
    getter code_language : String?
    # Optional override for [`ConversionOptions::autolinks`].
    getter autolinks : Bool?
    # Optional override for [`ConversionOptions::default_title`].
    getter default_title : Bool?
    # Optional override for [`ConversionOptions::br_in_tables`].
    getter br_in_tables : Bool?
    # Optional override for [`ConversionOptions::compact_tables`].
    getter compact_tables : Bool?
    # Optional override for [`ConversionOptions::highlight_style`].
    getter highlight_style : HighlightStyle?
    # Optional override for [`ConversionOptions::extract_metadata`].
    getter extract_metadata : Bool?
    # Optional override for [`ConversionOptions::whitespace_mode`].
    getter whitespace_mode : WhitespaceMode?
    # Optional override for [`ConversionOptions::strip_newlines`].
    getter strip_newlines : Bool?
    # Optional override for [`ConversionOptions::wrap`].
    getter wrap : Bool?
    # Optional override for [`ConversionOptions::wrap_width`].
    getter wrap_width : UInt64?
    # Optional override for [`ConversionOptions::convert_as_inline`].
    getter convert_as_inline : Bool?
    # Optional override for [`ConversionOptions::sub_symbol`].
    getter sub_symbol : String?
    # Optional override for [`ConversionOptions::sup_symbol`].
    getter sup_symbol : String?
    # Optional override for [`ConversionOptions::newline_style`].
    getter newline_style : NewlineStyle?
    # Optional override for [`ConversionOptions::code_block_style`].
    getter code_block_style : CodeBlockStyle?
    # Optional override for [`ConversionOptions::keep_inline_images_in`].
    getter keep_inline_images_in : Array(String)?
    # Optional override for [`ConversionOptions::preprocessing`].
    getter preprocessing : PreprocessingOptionsUpdate?
    # Optional override for [`ConversionOptions::encoding`].
    getter encoding : String?
    # Optional override for [`ConversionOptions::debug`].
    getter debug : Bool?
    # Optional override for [`ConversionOptions::strip_tags`].
    getter strip_tags : Array(String)?
    # Optional override for [`ConversionOptions::preserve_tags`].
    getter preserve_tags : Array(String)?
    # Optional override for [`ConversionOptions::skip_images`].
    getter skip_images : Bool?
    # Optional override for [`ConversionOptions::url_escape_style`].
    getter url_escape_style : UrlEscapeStyle?
    # Optional override for [`ConversionOptions::link_style`].
    getter link_style : LinkStyle?
    # Optional override for [`ConversionOptions::output_format`].
    getter output_format : OutputFormat?
    # Optional override for [`ConversionOptions::include_document_structure`].
    getter include_document_structure : Bool?
    # Optional override for [`ConversionOptions::extract_images`].
    getter extract_images : Bool?
    # Optional override for [`ConversionOptions::max_image_size`].
    getter max_image_size : UInt64?
    # Optional override for [`ConversionOptions::capture_svg`].
    getter capture_svg : Bool?
    # Optional override for [`ConversionOptions::infer_dimensions`].
    getter infer_dimensions : Bool?
    # Optional override for [`ConversionOptions::max_depth`].
    getter max_depth : UInt64?
    # Optional override for [`ConversionOptions::exclude_selectors`].
    getter exclude_selectors : Array(String)?
    # Optional override for [`ConversionOptions::tier_strategy`].
    getter tier_strategy : TierStrategy?
    # Optional override for [`ConversionOptions::visitor`].
    @[JSON::Field(ignore: true)]
    getter visitor : VisitorHandle?
  end

  # HTML preprocessing options for document cleanup before conversion.
  class PreprocessingOptions
    include JSON::Serializable
    # Enable HTML preprocessing globally
    getter enabled : Bool = true
    # Preprocessing preset level (Minimal, Standard, Aggressive)
    getter preset : PreprocessingPreset = PreprocessingPreset::Standard
    # Remove navigation elements (nav, breadcrumbs, menus, sidebars)
    getter remove_navigation : Bool = true
    # Remove form elements (forms, inputs, buttons, etc.)
    getter remove_forms : Bool = true
  end

  # Partial update for `PreprocessingOptions`.
  #
  # This struct uses `Option<T>` to represent optional fields that can be selectively updated.
  # Only specified fields (Some values) will override existing options; None values leave the
  # corresponding fields unchanged when applied via [`PreprocessingOptions::apply_update`].
  class PreprocessingOptionsUpdate
    include JSON::Serializable
    # Optional global preprocessing enablement override
    getter enabled : Bool?
    # Optional preprocessing preset level override (Minimal, Standard, Aggressive)
    getter preset : PreprocessingPreset?
    # Optional navigation element removal override (nav, breadcrumbs, menus, sidebars)
    getter remove_navigation : Bool?
    # Optional form element removal override (forms, inputs, buttons, etc.)
    getter remove_forms : Bool?
  end

  # Image dimensions in pixels.
  #
  # Binding-safe replacement for `(u32, u32)` tuples, which degrade to
  # `Vec<Vec<String>>` when sanitized for cross-language binding generation.
  # Used by both `ImageMetadata` and
  # `InlineImage`.
  class ImageDimensions
    include JSON::Serializable
    # Width in pixels.
    getter width : UInt32 = 0
    # Height in pixels.
    getter height : UInt32 = 0
  end

  # A structured document tree representing the semantic content of an HTML document.
  #
  # Uses a flat node array with index-based parent/child references for efficient traversal.
  class DocumentStructure
    include JSON::Serializable
    # All nodes in document reading order.
    getter nodes : Array(DocumentNode) = [] of DocumentNode
    # The source format (always "html" for this crate).
    getter source_format : String?
  end

  # A single node in the document tree.
  class DocumentNode
    include JSON::Serializable
    # Deterministic node identifier.
    getter id : String = ""
    # The semantic content of this node.
    getter content : NodeContent
    # Index of the parent node (None for root nodes).
    getter parent : UInt32?
    # Indices of child nodes in reading order.
    getter children : Array(UInt32) = [] of UInt32
    # Inline formatting annotations (bold, italic, links, etc.) with byte offsets into the text.
    getter annotations : Array(TextAnnotation) = [] of TextAnnotation
    # Format-specific attributes preserved from the source HTML element.
    #
    # Keys are lowercased attribute names as they appear in the HTML (e.g. `"class"`, `"id"`,
    # `"data-foo"`). Values are the raw attribute strings, copied verbatim from the source —
    # no HTML entity decoding is applied here.
    #
    # The map is `None` when no attributes are present (omitted entirely in serialized output).
    # Not every HTML attribute is preserved: only attributes that carry semantic or structural
    # significance for the node type are collected. For example, heading nodes capture the `"id"`
    # attribute for anchor linking; other element-level attributes may be silently dropped.
    getter attributes : Hash(String, String)?
  end

  # A styling or semantic annotation that applies to a byte range within a node's text.
  #
  # Unlike [`DocumentNode`], which captures block-level structure (headings, paragraphs, etc.),
  # a `TextAnnotation` describes inline-level markup — bold, italic, links, code spans, and
  # similar — that spans a contiguous run of bytes inside `DocumentNode::content`'s text field.
  #
  # Byte offsets (`start`..`end`) are into the UTF-8 encoded text of the parent node. The range
  # is half-open: `start` is inclusive and `end` is exclusive.
  #
  # Multiple annotations on the same node can overlap (e.g. bold-italic text), and they are
  # stored in the order they are encountered during DOM traversal.
  #
  # See [`AnnotationKind`] for the full list of supported annotation types.
  class TextAnnotation
    include JSON::Serializable
    # Start byte offset (inclusive) into the parent node's text.
    getter start : UInt32 = 0
    # End byte offset (exclusive) into the parent node's text.
    @[JSON::Field(key: "end")]
    getter end_ : UInt32 = 0
    # The type of annotation.
    getter kind : AnnotationKind
  end

  # A single key-value metadata entry from `<head>` meta tags.
  #
  # Binding-safe replacement for `(String, String)` tuples used in
  # [`NodeContent::MetadataBlock`]. Tuple pairs cannot be represented
  # across language boundaries without lossy degradation.
  class MetadataEntry
    include JSON::Serializable
    # Metadata key (e.g. `"title"`, `"description"`, `"og:title"`).
    getter key : String = ""
    # Metadata value.
    getter value : String = ""
  end

  # The primary result of HTML conversion and extraction.
  #
  # Contains the converted text output, optional structured document tree,
  # metadata, extracted tables, images, and processing warnings.
  # Example:
  #   ```crystal
  #   use html_to_markdown_rs::{convert, ConversionOptions};
  #
  #   let result = convert("<h1>Hello</h1><p>World</p>", None)?;
  #   assert!(result.content.is_some());
  #   assert!(result.warnings.is_empty());
  #   ```crystal
  class ConversionResult
    include JSON::Serializable
    # Converted text output in the selected format: Markdown, Djot, or plain text.
    getter content : String?
    # Structured document tree with semantic elements.
    #
    # Populated when `ConversionOptions::include_document_structure` is `true`. `None`
    # otherwise (the default), which avoids the overhead of building the tree.
    #
    # When present, the tree mirrors the converted document: headings open
    # `Group` sections, paragraphs and list items carry
    # inline `TextAnnotation`s, and tables reference the same
    # `TableGrid` data exposed in [`Self::tables`].
    #
    # Note: this field is independent of the `metadata` feature flag. Document structure
    # collection is always available at runtime; it is gated only by the runtime option, not
    # by a compile-time feature.
    getter document : DocumentStructure?
    # Extracted HTML metadata (title, OG, links, images, structured data).
    getter metadata : HtmlMetadata = HtmlMetadata.from_json("{}")
    # Extracted tables with structured cell data and markdown representation.
    getter tables : Array(TableData) = [] of TableData
    # Non-fatal processing warnings.
    getter warnings : Array(ProcessingWarning) = [] of ProcessingWarning
  end

  # A structured table grid with cell-level data including spans.
  class TableGrid
    include JSON::Serializable
    # Number of rows.
    getter rows : UInt32 = 0
    # Number of columns.
    getter cols : UInt32 = 0
    # All cells in the table as a flat, sparse list.
    #
    # The list is ordered by `(row, col)` but is **not** a dense `rows × cols` matrix: cells
    # that are covered by a spanning cell (via `row_span > 1` or `col_span > 1`) do not appear
    # in the list. Only the top-left "origin" cell of a span is present, with its `row_span`
    # and `col_span` fields set accordingly.
    #
    # To reconstruct the full visual grid, iterate over all cells and mark the rectangular
    # region `[row .. row+row_span, col .. col+col_span]` as occupied by that cell. Any
    # `(row, col)` position that is not the origin of any cell is covered by a span from an
    # earlier cell.
    #
    # The length of this vec is `≤ rows * cols`. An empty table (`rows == 0 || cols == 0`)
    # produces an empty vec.
    getter cells : Array(GridCell) = [] of GridCell
  end

  # A single cell in a table grid.
  class GridCell
    include JSON::Serializable
    # The text content of the cell.
    getter content : String = ""
    # 0-indexed row position.
    getter row : UInt32 = 0
    # 0-indexed column position.
    getter col : UInt32 = 0
    # Number of rows this cell spans (default 1).
    getter row_span : UInt32 = 0
    # Number of columns this cell spans (default 1).
    getter col_span : UInt32 = 0
    # Whether this is a header cell (`<th>`).
    getter is_header : Bool = false
  end

  # A top-level extracted table with both structured data and markdown representation.
  class TableData
    include JSON::Serializable
    # The structured table grid.
    getter grid : TableGrid = TableGrid.from_json("{}")
    # The markdown rendering of this table.
    getter markdown : String = ""
  end

  # A non-fatal diagnostic produced during HTML conversion.
  #
  # Warnings indicate that conversion completed but some content may have been handled
  # differently than expected — for example, an image that could not be extracted, a truncated
  # input, or malformed HTML that was repaired with best-effort parsing.
  #
  # Conversion always succeeds (returns `ConversionResult`) even when warnings are
  # present. Callers should inspect `warnings` and decide how to
  # handle them based on their tolerance for partial results:
  #
  # - **Logging pipelines**: emit each warning at `WARN` level and continue.
  # - **Strict pipelines**: treat any warning as a hard error by checking
  #   `result.warnings.is_empty()` before using the output.
  #
  # See [`WarningKind`] for the full taxonomy of warning categories.
  class ProcessingWarning
    include JSON::Serializable
    # Human-readable warning message.
    getter message : String = ""
    # The category of warning.
    getter kind : WarningKind = WarningKind::ImageExtractionFailed
  end

  # Shareable, thread-safe handle to a user-provided HTML visitor implementation.
  #
  # Pass an instance wrapped in this handle to `ConversionOptions` to
  # customise how the HTML document is traversed and converted to Markdown.
  # The handle may be cloned and shared across threads without additional
  # synchronisation on the caller's side.
  class VisitorHandle
    # Wraps the owned FFI handle; do not construct directly.
    def initialize(@handle : Void*)
    end
    # Raw handle for passing back across the C ABI.
    def to_unsafe : Void*
      @handle
    end
    def finalize
      LibHtm.visitor_handle_free(@handle) unless @handle.null?
    end
  end

  # Context information passed to all visitor methods.
  #
  # Provides comprehensive metadata about the current node being visited,
  # including its type, tag name, position in the DOM tree, and parent context.
  #
  # ## Attributes
  #
  # Access attributes via [`NodeContext::attributes`], which returns
  # `&BTreeMap<String, String>`. When the context was built with
  # [`NodeContext::with_lazy_attributes`] (the hot path inside the converter),
  # the map is only materialized on the first call — if the visitor never reads
  # attributes, the allocation is skipped.
  #
  # ## Lifetimes
  #
  # String fields use [`Cow<'_, str>`] so the converter can pass slices directly
  # out of the parsed DOM without allocating. Visitor implementations that need
  # to outlive the callback should call [`NodeContext::into_owned`].
  class NodeContext
    include JSON::Serializable
    # Coarse-grained node type classification
    getter node_type : NodeType = NodeType::Text
    # Raw HTML tag name (e.g., "div", "h1", "custom-element")
    getter tag_name : String = ""
    # Depth in the DOM tree (0 = root)
    getter depth : UInt64 = 0
    # Index among siblings (0-based)
    getter index_in_parent : UInt64 = 0
    # Parent element's tag name (None if root)
    getter parent_tag : String?
    # Whether this element is treated as inline vs block
    getter is_inline : Bool = false
  end

  # Text directionality of document content.
  #
  # Corresponds to the HTML `dir` attribute and `bdi` element directionality.
  enum TextDirection
    LeftToRight
    RightToLeft
    Auto
  end

  # Link classification based on href value and document context.
  #
  # Used to categorize links during extraction for filtering and analysis.
  enum LinkType
    Anchor
    Internal
    External
    Email
    Phone
    Other
  end

  # Image source classification for proper handling and processing.
  #
  # Determines whether an image is embedded (data URI), inline SVG, external, or relative.
  enum ImageType
    DataUri
    InlineSvg
    External
    Relative
  end

  # Structured data format type.
  #
  # Identifies the schema/format used for structured data markup.
  enum StructuredDataType
    JsonLd
    Microdata
    RdFa
  end

  # Controls which conversion tier is used.
  enum TierStrategy
    Auto
    Tier2
    Tier1
  end

  # HTML preprocessing aggressiveness level.
  #
  # Controls the extent of cleanup performed before conversion. Higher levels remove more elements.
  enum PreprocessingPreset
    Minimal
    Standard
    Aggressive
  end

  # Heading style options for Markdown output.
  #
  # Controls how headings (h1-h6) are rendered in the output Markdown.
  enum HeadingStyle
    Underlined
    Atx
    AtxClosed
  end

  # List indentation character type.
  #
  # Controls whether list items are indented with spaces or tabs.
  enum ListIndentType
    Spaces
    Tabs
  end

  # Whitespace handling strategy during conversion.
  #
  # Determines how sequences of whitespace characters (spaces, tabs, newlines) are processed.
  enum WhitespaceMode
    Normalized
    Strict
  end

  # Line break syntax in Markdown output.
  #
  # Controls how soft line breaks (from `<br>` or line breaks in source) are rendered.
  enum NewlineStyle
    Spaces
    Backslash
  end

  # Code block fence style in Markdown output.
  #
  # Determines how code blocks (`<pre><code>`) are rendered in Markdown.
  enum CodeBlockStyle
    Indented
    Backticks
    Tildes
  end

  # Highlight rendering style for `<mark>` elements.
  #
  # Controls how highlighted text is rendered in Markdown output.
  enum HighlightStyle
    DoubleEqual
    Html
    Bold
    None
  end

  # Link rendering style in Markdown output.
  #
  # Controls whether links and images use inline `[text](url)` syntax or
  # reference-style `[text][1]` syntax with definitions collected at the end.
  enum LinkStyle
    Inline
    Reference
  end

  # URL encoding strategy for link and image destinations.
  #
  # Controls how special characters in URL destinations are handled when they
  # require escaping to produce valid Markdown.
  #
  # The `Angle` variant (default) wraps the destination in angle brackets:
  # `[text](<url with spaces>)`. This is the CommonMark-specified escape hatch
  # but breaks when the URL itself contains `>`.
  #
  # The `Percent` variant percent-encodes every character that is not an RFC 3986
  # unreserved character or `/`, producing a destination safe for all Markdown
  # parsers: `[text](url%20with%20spaces)`.
  enum UrlEscapeStyle
    Angle
    Percent
  end

  # Output format for conversion.
  #
  # Specifies the target markup language format for the conversion output.
  enum OutputFormat
    Markdown
    Djot
    Plain
  end

  # The semantic content type of a document node.
  #
  # Uses internally tagged representation (`"node_type": "heading"`) for JSON serialization.
  abstract class NodeContent
    include JSON::Serializable
    use_json_discriminator "node_type", {"heading" => NodeContent::Heading, "paragraph" => NodeContent::Paragraph, "list" => NodeContent::List, "list_item" => NodeContent::ListItem, "table" => NodeContent::Table, "image" => NodeContent::Image, "code" => NodeContent::Code, "quote" => NodeContent::Quote, "definition_list" => NodeContent::DefinitionList, "definition_item" => NodeContent::DefinitionItem, "raw_block" => NodeContent::RawBlock, "metadata_block" => NodeContent::MetadataBlock, "group" => NodeContent::Group}
  end

  class NodeContent::Heading < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "heading"
    getter level : UInt8
    getter text : String
  end

  class NodeContent::Paragraph < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "paragraph"
    getter text : String
  end

  class NodeContent::List < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "list"
    getter ordered : Bool
  end

  class NodeContent::ListItem < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "list_item"
    getter text : String
  end

  class NodeContent::Table < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "table"
    getter grid : TableGrid
  end

  class NodeContent::Image < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "image"
    getter description : String?
    getter src : String?
    getter image_index : UInt32?
  end

  class NodeContent::Code < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "code"
    getter text : String
    getter language : String?
  end

  class NodeContent::Quote < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "quote"
  end

  class NodeContent::DefinitionList < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "definition_list"
  end

  class NodeContent::DefinitionItem < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "definition_item"
    getter term : String
    getter definition : String
  end

  class NodeContent::RawBlock < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "raw_block"
    getter format : String
    getter content : String
  end

  class NodeContent::MetadataBlock < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "metadata_block"
    getter entries : Array(MetadataEntry)
  end

  class NodeContent::Group < NodeContent
    include JSON::Serializable
    @[JSON::Field(key: "node_type")]
    getter node_type : String = "group"
    getter label : String?
    getter heading_level : UInt8?
    getter heading_text : String?
  end

  # The type of an inline text annotation.
  #
  # Uses internally tagged representation (`"annotation_type": "bold"`) for JSON serialization.
  abstract class AnnotationKind
    include JSON::Serializable
    use_json_discriminator "annotation_type", {"bold" => AnnotationKind::Bold, "italic" => AnnotationKind::Italic, "underline" => AnnotationKind::Underline, "strikethrough" => AnnotationKind::Strikethrough, "code" => AnnotationKind::Code, "subscript" => AnnotationKind::Subscript, "superscript" => AnnotationKind::Superscript, "highlight" => AnnotationKind::Highlight, "link" => AnnotationKind::Link}
  end

  class AnnotationKind::Bold < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "bold"
  end

  class AnnotationKind::Italic < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "italic"
  end

  class AnnotationKind::Underline < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "underline"
  end

  class AnnotationKind::Strikethrough < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "strikethrough"
  end

  class AnnotationKind::Code < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "code"
  end

  class AnnotationKind::Subscript < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "subscript"
  end

  class AnnotationKind::Superscript < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "superscript"
  end

  class AnnotationKind::Highlight < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "highlight"
  end

  class AnnotationKind::Link < AnnotationKind
    include JSON::Serializable
    @[JSON::Field(key: "annotation_type")]
    getter annotation_type : String = "link"
    getter url : String
    getter title : String?
  end

  # Categories of processing warnings.
  enum WarningKind
    ImageExtractionFailed
    EncodingFallback
    TruncatedInput
    MalformedHtml
    SanitizationApplied
    DepthLimitExceeded
  end

  # Node type enumeration covering all HTML element types.
  #
  # This enum categorizes all HTML elements that the converter recognizes,
  # providing a coarse-grained classification for visitor dispatch.
  enum NodeType
    Text
    Element
    Heading
    Paragraph
    Div
    Blockquote
    Pre
    Hr
    List
    ListItem
    DefinitionList
    DefinitionTerm
    DefinitionDescription
    Table
    TableRow
    TableCell
    TableHeader
    TableBody
    TableHead
    TableFoot
    Link
    Image
    Strong
    Em
    Code
    Strikethrough
    Underline
    Subscript
    Superscript
    Mark
    Small
    Br
    Span
    Article
    Section
    Nav
    Aside
    Header
    Footer
    Main
    Figure
    Figcaption
    Time
    Details
    Summary
    Form
    Input
    Select
    Option
    Button
    Textarea
    Label
    Fieldset
    Legend
    Audio
    Video
    Picture
    Source
    Iframe
    Svg
    Canvas
    Ruby
    Rt
    Rp
    Abbr
    Kbd
    Samp
    Var
    Cite
    Q
    Del
    Ins
    Data
    Meter
    Progress
    Output
    Template
    Slot
    Html
    Head
    Body
    Title
    Meta
    LinkTag
    Style
    Script
    Base
    Custom
  end

  # Result of a visitor callback.
  #
  # Allows visitors to control the conversion flow by either proceeding
  # with default behavior, providing custom output, skipping elements,
  # preserving HTML, or signaling errors.
  abstract class VisitResult
    def self.new(pull : ::JSON::PullParser) : VisitResult
      case pull.kind
      when .string?
        __tag = pull.read_string
        case __tag
        when "Continue" then return VisitResult::Continue.new
        when "Skip" then return VisitResult::Skip.new
        when "PreserveHtml" then return VisitResult::PreserveHtml.new
        else raise ::JSON::ParseException.new("unknown VisitResult variant: #{__tag}", *pull.location)
        end
      when .begin_object?
        __result : VisitResult? = nil
        pull.read_object do |__key|
          case __key
          when "Custom" then __result = VisitResult::Custom.new(pull)
          when "Error" then __result = VisitResult::Error.new(pull)
          else pull.skip
          end
        end
        return __result || raise ::JSON::ParseException.new("empty VisitResult object", *pull.location)
      else
        raise ::JSON::ParseException.new("invalid VisitResult JSON", *pull.location)
      end
    end

    def self.from_json(string : String) : VisitResult
      new(::JSON::PullParser.new(string))
    end

    abstract def to_json(json : ::JSON::Builder)
  end

  class VisitResult::Continue < VisitResult
    def to_json(json : ::JSON::Builder)
      json.string("Continue")
    end
  end

  class VisitResult::Custom < VisitResult
    getter value : String
    def initialize(@value : String)
    end
    def self.new(pull : ::JSON::PullParser) : VisitResult::Custom
      __v = String.new(pull)
      new(__v)
    end
    def to_json(json : ::JSON::Builder)
      json.object do
        json.field("Custom") do
          @value.to_json(json)
        end
      end
    end
  end

  class VisitResult::Skip < VisitResult
    def to_json(json : ::JSON::Builder)
      json.string("Skip")
    end
  end

  class VisitResult::PreserveHtml < VisitResult
    def to_json(json : ::JSON::Builder)
      json.string("PreserveHtml")
    end
  end

  class VisitResult::Error < VisitResult
    getter value : String
    def initialize(@value : String)
    end
    def self.new(pull : ::JSON::PullParser) : VisitResult::Error
      __v = String.new(pull)
      new(__v)
    end
    def to_json(json : ::JSON::Builder)
      json.object do
        json.field("Error") do
          @value.to_json(json)
        end
      end
    end
  end

  # Errors that can occur during HTML to Markdown conversion.
  class ConversionError < Exception
  end

  # Convert HTML to Markdown, Djot, or plain text.
  def self.convert(html : String, options : ConversionOptions?) : ConversionResult
    __handle_options = options.nil? ? Pointer(LibHtm::ConversionOptions).null : LibHtm.conversion_options_from_json(options.not_nil!.to_json)
    __ptr = LibHtm.convert(html, __handle_options)
    raise "LibHtm.convert returned a null pointer" if __ptr.null?
    __json_ptr = LibHtm.conversion_result_to_json(__ptr)
    LibHtm.conversion_result_free(__ptr)
    __json = String.new(__json_ptr)
    LibHtm.free_string(__json_ptr)
    LibHtm.conversion_options_free(__handle_options) unless __handle_options.null?
    ConversionResult.from_json(__json)
  end
end
require "./html_to_markdown_rs_html_visitor_visitor"
