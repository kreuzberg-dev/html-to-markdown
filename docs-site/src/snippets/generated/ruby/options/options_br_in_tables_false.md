```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>', HtmlToMarkdownRs::ConversionOptions.new(br_in_tables: false))

```
