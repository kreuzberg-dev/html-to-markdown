```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_line_break(*args)
    { Custom: ' | ' }
  end
end.new
result = HtmlToMarkdown.convert('<p>First line<br>Second line<br>Third line</p>', visitor)

```
