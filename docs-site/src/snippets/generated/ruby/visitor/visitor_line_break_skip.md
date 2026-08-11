---
id: fixture_ruby_visitor_line_break_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_line_break(*args)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>', visitor)

```
