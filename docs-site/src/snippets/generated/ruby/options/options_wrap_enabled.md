```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>', HtmlToMarkdownRs::ConversionOptions.new(wrap: true, wrap_width: 40))

```
