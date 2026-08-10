```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_emphasis(ctx, text)
    { Custom: ">>>#{text}<<<" }
  end
end.new
result = HtmlToMarkdown.convert('<p>This is <em>important</em> text.</p>', visitor)

```
