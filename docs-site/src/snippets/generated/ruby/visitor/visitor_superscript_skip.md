---
id: fixture_ruby_visitor_superscript_skip
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
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>', visitor)

```
