```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_input(ctx, input_type, name, value)
    { Custom: "[INPUT:#{input_type}]" }
  end
end.new
result = HtmlToMarkdown.convert('<form><label>Username: <input type="text" name="username" value=""></label><label>Password: <input type="password" name="password"></label></form>', visitor)

```
