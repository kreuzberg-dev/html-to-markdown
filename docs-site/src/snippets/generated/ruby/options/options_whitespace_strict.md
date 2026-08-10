```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Preserved   spacing.</p>', HtmlToMarkdownRs::ConversionOptions.new(whitespace_mode: 'Strict'))

```
