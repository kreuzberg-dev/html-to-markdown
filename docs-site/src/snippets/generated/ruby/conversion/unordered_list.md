---
id: fixture_ruby_unordered_list
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>')

```
