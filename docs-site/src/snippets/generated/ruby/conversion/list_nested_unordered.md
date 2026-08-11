---
id: fixture_ruby_list_nested_unordered
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>')

```
