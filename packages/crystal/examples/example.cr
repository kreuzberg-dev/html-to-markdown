# html-to-markdown Crystal — convert HTML to Markdown
require "../src/html_to_markdown_rs"

# Minimal config — enum and struct fields must be provided since Crystal
# doesn't infer defaults for them from JSON (bool/int/array/hash/String
# fields all get `getter field : Type = default` in the generated binding).
def default_options : HtmlToMarkdown::ConversionOptions
  HtmlToMarkdown::ConversionOptions.from_json(%({
    "heading_style":"Atx",
    "list_indent_type":"Spaces",
    "highlight_style":"None",
    "whitespace_mode":"Normalized",
    "newline_style":"Spaces",
    "code_block_style":"Backticks",
    "url_escape_style":"Percent",
    "link_style":"Inline",
    "output_format":"Markdown",
    "tier_strategy":"Auto",
    "preprocessing":{"preset":"Standard"}
  }))
end

# Convert with default options
puts "=== Default conversion ==="
result = HtmlToMarkdown.convert(
  "<h1>Hello World</h1><p>This is <strong>markdown</strong>.</p>",
  default_options
)
puts result.content

# Convert with custom options
puts "\n=== With plain text output ==="
options = HtmlToMarkdown::ConversionOptions.from_json(%({
  "heading_style":"Atx",
  "list_indent_type":"Spaces",
  "highlight_style":"None",
  "whitespace_mode":"Normalized",
  "newline_style":"Spaces",
  "code_block_style":"Backticks",
  "url_escape_style":"Percent",
  "link_style":"Inline",
  "output_format":"Plain",
  "tier_strategy":"Auto",
  "preprocessing":{"preset":"Standard"}
}))
result = HtmlToMarkdown.convert(
  "<h1>Title</h1><p>Some long paragraph text.</p>",
  options
)
puts result.content

puts "\nCrystal html-to-markdown bindings working!"
