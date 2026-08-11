---
id: fixture_ruby_visitor_button_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_button(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>', visitor)

```
