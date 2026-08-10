```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text<img src="data:BADMIME" alt="broken">end</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_images: true))

```
