```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Line one<br>Line two</p>', HtmlToMarkdownRs::ConversionOptions.new(newline_style: 'Backslash'))

```
