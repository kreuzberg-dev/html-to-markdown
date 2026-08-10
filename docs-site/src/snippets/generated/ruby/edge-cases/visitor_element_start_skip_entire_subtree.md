```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_element_start(*args)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<div><h1>Title</h1><p>Content</p></div>', visitor)

```
