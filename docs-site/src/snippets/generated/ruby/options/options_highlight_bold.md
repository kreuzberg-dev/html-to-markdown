```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>highlighted</mark> text.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'Bold'))

```
