```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Before SVG.</p><svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>")

```
