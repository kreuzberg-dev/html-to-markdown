---
id: fixture_ruby_line_break_br_tag
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>First line.<br>Second line.</p>')

```
