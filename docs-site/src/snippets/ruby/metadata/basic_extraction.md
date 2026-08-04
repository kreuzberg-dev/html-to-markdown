```ruby
require 'html_to_markdown'

html = '<html lang="en"><head><title>Test</title></head><body><h1>Hello</h1></body></html>'
result = HtmlToMarkdown.convert(html, extract_metadata: true)

markdown = result.content
puts result.metadata.document.title     # "Test"
puts result.metadata.headers.first.text # "Hello"
```
