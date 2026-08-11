---
id: fixture_ruby_image_simple
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<img src="photo.jpg" alt="A photo">')

```
