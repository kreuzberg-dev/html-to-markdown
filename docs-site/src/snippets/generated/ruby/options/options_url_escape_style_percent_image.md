```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<img src="/img (1) <draft>.png" alt="alt">', HtmlToMarkdownRs::ConversionOptions.new(url_escape_style: 'percent'))

```
