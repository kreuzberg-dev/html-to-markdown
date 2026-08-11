---
id: fixture_ruby_visitor_definition_list_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_definition_term(ctx, text)
    { Custom: "**#{text}**" }
  end
end.new
result = HtmlToMarkdown.convert('<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>', visitor)

```
