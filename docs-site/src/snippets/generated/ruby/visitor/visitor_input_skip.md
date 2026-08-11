---
id: fixture_ruby_visitor_input_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_input(ctx, input_type, name, value)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Sign up:</p><input type="text" name="email" placeholder="your@email.com"><input type="checkbox" name="agree"><p>Continue</p>', visitor)

```
