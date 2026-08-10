```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>plain</mark> content.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'None'))

```
