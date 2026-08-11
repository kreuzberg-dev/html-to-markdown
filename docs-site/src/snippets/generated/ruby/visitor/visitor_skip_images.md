---
id: fixture_ruby_visitor_skip_images
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_image(ctx, src, alt, title)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Before image</p><img src="photo.jpg" alt="A photo"><p>After image</p>', visitor)

```
