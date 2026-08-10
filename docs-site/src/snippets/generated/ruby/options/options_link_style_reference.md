```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", HtmlToMarkdownRs::ConversionOptions.new(link_style: 'Reference'))

```
