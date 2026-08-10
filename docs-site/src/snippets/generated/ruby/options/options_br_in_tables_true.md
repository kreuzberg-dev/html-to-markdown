```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>', HtmlToMarkdownRs::ConversionOptions.new(br_in_tables: true))

```
