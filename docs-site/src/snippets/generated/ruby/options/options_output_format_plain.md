```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Title</h1><p>Some <strong>bold</strong> text.</p>', HtmlToMarkdownRs::ConversionOptions.new(output_format: 'Plain'))

```
