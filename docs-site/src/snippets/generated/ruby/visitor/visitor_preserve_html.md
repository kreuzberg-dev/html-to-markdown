---
id: fixture_ruby_visitor_preserve_html
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
    'PreserveHtml'
  end
end.new
result = HtmlToMarkdown.convert('<div><custom-tag>Custom content</custom-tag></div>', visitor)

```
