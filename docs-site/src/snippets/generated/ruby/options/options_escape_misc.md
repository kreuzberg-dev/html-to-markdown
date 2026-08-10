```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Use # and | and ~ in text.</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_misc: true))

```
