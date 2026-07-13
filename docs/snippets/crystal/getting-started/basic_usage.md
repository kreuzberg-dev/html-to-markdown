```crystal title="Crystal"
require "html_to_markdown_rs"

# Convert HTML to Markdown with default options
result = HtmlToMarkdownRs.convert("<h1>Hello World</h1>", HtmlToMarkdownRs::ConversionOptions.from_json("null"))
puts result.content  # => "# Hello World"

# With custom options
options = HtmlToMarkdownRs::ConversionOptions.from_json(%({"output_format":"plain"}))
result = HtmlToMarkdownRs.convert("<p>Hello</p>", options)
puts result.content  # => "Hello"
```
