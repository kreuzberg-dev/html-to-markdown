---
id: fixture_ruby_semantic_hr
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Above</p><hr><p>Below</p>')

```
