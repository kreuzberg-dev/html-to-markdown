```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>My Page</title></head><body><p>Content</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
