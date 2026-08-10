```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Hello</p>', HtmlToMarkdownRs::ConversionOptions.new(max_depth: 0))

```
