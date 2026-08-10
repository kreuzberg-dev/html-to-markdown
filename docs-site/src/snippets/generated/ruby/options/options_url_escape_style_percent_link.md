```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="/file (1).pdf">file</a>', HtmlToMarkdownRs::ConversionOptions.new(url_escape_style: 'percent'))

```
