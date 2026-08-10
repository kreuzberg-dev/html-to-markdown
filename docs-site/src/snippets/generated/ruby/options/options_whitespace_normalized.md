```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text   with    extra   spaces.</p>', HtmlToMarkdownRs::ConversionOptions.new(whitespace_mode: 'Normalized'))

```
