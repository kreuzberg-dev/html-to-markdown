---
id: fixture_ruby_visitor_underline_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_underline(ctx, text)
    { Custom: "_#{text}_" }
  end
end.new
result = HtmlToMarkdown.convert('<p>This is <u>very important</u> text.</p>', visitor)

```
