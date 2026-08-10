```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Title</h1><p>A paragraph of text.</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
