```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>First paragraph.</p><p>Second paragraph.</p>', HtmlToMarkdownRs::ConversionOptions.new(strip_newlines: true))

```
