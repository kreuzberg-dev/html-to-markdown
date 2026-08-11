---
id: fixture_ruby_visitor_figure_custom_wrap
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_figure_end(ctx, output, *args)
    { Custom: "#{output}
[/FIGURE]
" }
  end
  def visit_figure_start(*args)
    { Custom: "\n[FIGURE]\n" }
  end
end.new
result = HtmlToMarkdown.convert('<section><h2>Gallery</h2><figure><img src="photo1.jpg" alt="Photo"><figcaption>Beautiful sunset</figcaption></figure></section>', visitor)

```
