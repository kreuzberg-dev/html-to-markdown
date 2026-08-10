```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>https://example.com</a></p>", HtmlToMarkdownRs::ConversionOptions.new(autolinks: false))

```
