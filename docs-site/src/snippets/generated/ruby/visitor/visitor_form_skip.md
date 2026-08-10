```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_form(ctx, action_url, method)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Before form</p><form><input type="email" name="email"></form><p>After form</p>', visitor)

```
