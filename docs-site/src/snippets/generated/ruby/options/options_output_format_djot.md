```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Simple paragraph.</p>', HtmlToMarkdownRs::ConversionOptions.new(output_format: 'Djot'))

```
