```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
