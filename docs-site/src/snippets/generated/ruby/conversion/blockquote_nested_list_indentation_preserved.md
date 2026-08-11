---
id: fixture_ruby_blockquote_nested_list_indentation_preserved
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>')

```
