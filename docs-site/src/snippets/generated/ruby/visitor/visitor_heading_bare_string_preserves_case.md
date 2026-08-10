```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_heading(ctx, level, text, id)
    "## #{text} ##"
  end
end.new
result = HtmlToMarkdown.convert('<h2>Important Section Title</h2><p>Body.</p>', visitor)

```
