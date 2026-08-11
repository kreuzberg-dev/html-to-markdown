---
id: fixture_ruby_line_break_hr_tag
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Before rule.</p><hr><p>After rule.</p>')

```
