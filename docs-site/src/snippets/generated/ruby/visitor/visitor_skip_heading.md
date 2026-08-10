```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_heading(ctx, level, text, id)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<h1>Title</h1><p>Body text remains.</p>', visitor)

```
