```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Link</a></p>", HtmlToMarkdownRs::ConversionOptions.new(default_title: true))

```
