---
id: fixture_ruby_ordered_list
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ol><li>First</li><li>Second</li><li>Third</li></ol>')

```
