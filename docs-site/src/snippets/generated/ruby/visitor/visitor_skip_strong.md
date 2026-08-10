```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_strong(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Normal <strong>bold text</strong> normal</p>', visitor)

```
