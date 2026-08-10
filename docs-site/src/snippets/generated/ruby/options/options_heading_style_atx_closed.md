```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Closed Heading</h1>', HtmlToMarkdownRs::ConversionOptions.new(heading_style: 'AtxClosed'))

```
