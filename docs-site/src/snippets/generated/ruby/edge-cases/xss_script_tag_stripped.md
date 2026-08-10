```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>")

```
