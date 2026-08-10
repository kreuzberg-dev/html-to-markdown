```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_horizontal_rule(*args)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>', visitor)

```
