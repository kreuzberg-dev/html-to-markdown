---
id: fixture_ruby_link_image_inside
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="https://example.com"><img src="logo.png" alt="Logo"></a>')

```
