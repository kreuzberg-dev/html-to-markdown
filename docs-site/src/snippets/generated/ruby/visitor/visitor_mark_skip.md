---
id: fixture_ruby_visitor_mark_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_mark(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Key insight: <mark>always validate input</mark> for security.</p>', visitor)

```
