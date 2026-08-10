```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>See <a href="https://example.com">Example</a> for details.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
