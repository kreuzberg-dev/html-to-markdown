```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_link(ctx, href, text, title)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Before <a href="https://example.com">link text</a> after</p>', visitor)

```
