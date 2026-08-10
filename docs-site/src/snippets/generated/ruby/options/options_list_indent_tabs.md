```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Parent<ul><li>Child</li></ul></li></ul>', HtmlToMarkdownRs::ConversionOptions.new(list_indent_type: 'Tabs'))

```
