---
id: fixture_ruby_visitor_custom_element_with_nesting
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_custom_element(ctx, tag_name, html)
    { Custom: '[CUSTOM WIDGET]' }
  end
end.new
result = HtmlToMarkdown.convert('<div><custom-widget data-value="123"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>', visitor)

```
