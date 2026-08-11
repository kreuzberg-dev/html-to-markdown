---
id: fixture_ruby_visitor_superscript_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_superscript(ctx, text)
    { Custom: "^#{text}^" }
  end
end.new
result = HtmlToMarkdown.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", visitor)

```
