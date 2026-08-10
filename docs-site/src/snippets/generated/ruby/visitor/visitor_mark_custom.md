```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_mark(ctx, text)
    { Custom: "==#{text}==" }
  end
end.new
result = HtmlToMarkdown.convert('<p>This is a <mark>highlighted passage</mark> in the text.</p>', visitor)

```
