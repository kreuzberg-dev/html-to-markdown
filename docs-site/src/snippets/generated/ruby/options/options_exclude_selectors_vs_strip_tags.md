```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="wrapper"><p>Inner paragraph</p></div><p>Outer text</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.wrapper']))

```
