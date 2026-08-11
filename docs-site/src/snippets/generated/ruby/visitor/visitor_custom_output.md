---
id: fixture_ruby_visitor_custom_output
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_heading(ctx, level, text, id)
    { Custom: '## REPLACED HEADING' }
  end
end.new
result = HtmlToMarkdown.convert('<h1>Original Heading</h1>', visitor)

```
