```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with # hash and [brackets] and * star</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_ascii: true))

```
