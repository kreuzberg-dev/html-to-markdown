---
id: fixture_ruby_visitor_iframe_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_iframe(ctx, src)
    { Custom: '[EMBEDDED: https://maps.example.com/embed]' }
  end
end.new
result = HtmlToMarkdown.convert('<p>Embedded map:</p><iframe src="https://maps.example.com/embed" width="400" height="300"></iframe><p>End of map</p>', visitor)

```
