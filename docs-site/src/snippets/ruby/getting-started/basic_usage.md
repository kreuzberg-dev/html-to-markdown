```ruby
require 'html_to_markdown'

html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
result = HtmlToMarkdown.convert(html)
markdown = result.content
```
