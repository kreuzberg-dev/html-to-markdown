```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><img src="https://example.com/photo.jpg" alt="A photo"></p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
