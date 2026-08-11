---
id: fixture_ruby_paragraph_with_line_breaks
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Line one.<br>Line two.<br>Line three.</p>')

```
