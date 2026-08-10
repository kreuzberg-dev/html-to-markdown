```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Outer<ul><li>Inner</li></ul></li></ul>', HtmlToMarkdownRs::ConversionOptions.new(list_indent_width: 4))

```
