```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_link(ctx, href, text, title)
    { Custom: '[REDACTED LINK]' }
  end
end.new
result = HtmlToMarkdown.convert('<a href="https://example.com">Click here</a>', visitor)

```
