```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>highlighted</mark> here.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'DoubleEqual'))

```
