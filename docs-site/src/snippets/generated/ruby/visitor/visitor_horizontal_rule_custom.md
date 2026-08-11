---
id: fixture_ruby_visitor_horizontal_rule_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_horizontal_rule(*args)
    { Custom: "\n[DIVIDER]\n" }
  end
end.new
result = HtmlToMarkdown.convert('<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>', visitor)

```
