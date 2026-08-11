---
id: fixture_ruby_visitor_skip_links
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_link(ctx, href, text, title)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Before <a href="https://example.com">link text</a> after</p>', visitor)

```
