```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>First<br>Second</p>', HtmlToMarkdownRs::ConversionOptions.new(newline_style: 'Spaces'))

```
