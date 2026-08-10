```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>The variable_name is defined.</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_underscores: true))

```
