```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
