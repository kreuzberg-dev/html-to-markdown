```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_underline(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Normal text with <u>underlined part</u> and more text.</p>', visitor)

```
