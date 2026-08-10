```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>No tables here</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
