---
id: fixture_ruby_visitor_element_end_modification
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_element_end(ctx, output, *args)
    { Custom: 'MODIFIED OUTPUT' }
  end
end.new
result = HtmlToMarkdown.convert('<blockquote><p>Original quote</p></blockquote>', visitor)

```
