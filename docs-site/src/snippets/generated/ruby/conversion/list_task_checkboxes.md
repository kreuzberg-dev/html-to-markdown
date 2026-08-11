---
id: fixture_ruby_list_task_checkboxes
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li><input type="checkbox" checked> Done task</li><li><input type="checkbox"> Pending task</li></ul>')

```
