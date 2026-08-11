---
id: fixture_ruby_result_tables_without_structure_flag
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>')

```
