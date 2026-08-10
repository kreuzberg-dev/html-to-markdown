```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
