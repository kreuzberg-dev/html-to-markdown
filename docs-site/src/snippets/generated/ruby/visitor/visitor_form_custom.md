---
id: fixture_ruby_visitor_form_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_form(ctx, action_url, method)
    { Custom: '[FORM PLACEHOLDER]' }
  end
end.new
result = HtmlToMarkdown.convert('<div><form action="/submit" method="POST"><label>Name: <input type="text" name="name"></label><button type="submit">Submit</button></form></div>', visitor)

```
