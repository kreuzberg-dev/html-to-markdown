---
id: fixture_ruby_visitor_heading_bare_string_preserves_case
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
    "## #{text} ##"
  end
end.new
result = HtmlToMarkdown.convert('<h2>Important Section Title</h2><p>Body.</p>', visitor)

```
