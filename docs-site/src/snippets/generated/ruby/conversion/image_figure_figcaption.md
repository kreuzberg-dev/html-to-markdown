---
id: fixture_ruby_image_figure_figcaption
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<figure><img src="sunset.jpg" alt="A sunset"><figcaption>Beautiful sunset over the ocean</figcaption></figure>')

```
