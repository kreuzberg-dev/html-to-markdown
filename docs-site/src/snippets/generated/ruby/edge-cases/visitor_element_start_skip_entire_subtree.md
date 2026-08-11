---
id: fixture_ruby_visitor_element_start_skip_entire_subtree
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_element_start(*args)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<div><h1>Title</h1><p>Content</p></div>', visitor)

```
