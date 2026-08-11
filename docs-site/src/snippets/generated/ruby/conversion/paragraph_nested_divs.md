---
id: fixture_ruby_paragraph_nested_divs
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<div><div><p>Nested text</p></div></div>')

```
