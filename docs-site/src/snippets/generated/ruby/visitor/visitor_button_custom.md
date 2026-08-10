```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_button(ctx, text)
    { Custom: "[BTN:#{text}]" }
  end
end.new
result = HtmlToMarkdown.convert('<p>Confirm action: <button type="submit">Click me</button> or <button type="reset">Cancel</button></p>', visitor)

```
