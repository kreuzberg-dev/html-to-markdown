---
id: fixture_ruby_line_break_multiple_br
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Start.<br><br>End.</p>')

```
