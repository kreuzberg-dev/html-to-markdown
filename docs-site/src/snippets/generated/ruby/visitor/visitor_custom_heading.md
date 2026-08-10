```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_heading(ctx, level, text, id)
    { Custom: "--- #{text} ---" }
  end
end.new
result = HtmlToMarkdown.convert('<h2>Section Title</h2><p>Content below heading.</p>', visitor)

```
