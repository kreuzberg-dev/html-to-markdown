---
id: fixture_ruby_table_with_colspan
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><thead><tr><th colspan="2">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>')

```
