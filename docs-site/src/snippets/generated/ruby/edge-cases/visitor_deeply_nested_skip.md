```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_mark(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>', visitor)

```
