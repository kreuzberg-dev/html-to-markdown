```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_summary(ctx, text)
    { Custom: "[EXPANDABLE] #{text}" }
  end
end.new
result = HtmlToMarkdown.convert('<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>', visitor)

```
