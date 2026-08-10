```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Item A</li><li>Item B</li></ul>', HtmlToMarkdownRs::ConversionOptions.new(bullets: '*'))

```
