```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_subscript(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>', visitor)

```
