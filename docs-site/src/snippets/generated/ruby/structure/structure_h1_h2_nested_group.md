```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
