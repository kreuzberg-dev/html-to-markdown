---
id: fixture_ruby_visitor_continue_default
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_strong(ctx, text)
    'Continue'
  end
end.new
result = HtmlToMarkdown.convert('<p>Hello <strong>World</strong></p>', visitor)

```
