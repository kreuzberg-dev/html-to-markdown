---
id: fixture_ruby_visitor_custom_blockquote
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_blockquote(ctx, content, depth)
    { Custom: "QUOTE: \"#{content}\"" }
  end
end.new
result = HtmlToMarkdown.convert('<blockquote><p>A wise quote.</p></blockquote>', visitor)

```
