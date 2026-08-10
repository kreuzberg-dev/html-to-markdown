```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Main Title</h1>', HtmlToMarkdownRs::ConversionOptions.new(heading_style: 'Underlined'))

```
