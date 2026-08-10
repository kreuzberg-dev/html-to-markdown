```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
