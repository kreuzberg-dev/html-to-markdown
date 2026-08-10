```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_iframe(ctx, src)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<h3>Reviews</h3><iframe src="https://widget.example.com/reviews"></iframe><p>See reviews from our partners.</p>', visitor)

```
