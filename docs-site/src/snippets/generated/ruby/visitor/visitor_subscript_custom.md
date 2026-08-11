---
id: fixture_ruby_visitor_subscript_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_subscript(ctx, text)
    { Custom: "~#{text}~" }
  end
end.new
result = HtmlToMarkdown.convert('<p>H<sub>2</sub>O is water.</p>', visitor)

```
