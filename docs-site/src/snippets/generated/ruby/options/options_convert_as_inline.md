```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>One</p><p>Two</p>', HtmlToMarkdownRs::ConversionOptions.new(convert_as_inline: true))

```
