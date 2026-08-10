```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
