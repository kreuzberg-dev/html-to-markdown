```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<article><h1>Heading</h1><p>Paragraph body.</p></article>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
