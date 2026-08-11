---
id: fixture_ruby_visitor_figure_custom
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_figcaption(ctx, text)
    { Custom: "*#{text}*" }
  end
end.new
result = HtmlToMarkdown.convert('<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src="diagram.png" alt="System architecture diagram"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>', visitor)

```
