```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Hello world</p>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: []))

```
