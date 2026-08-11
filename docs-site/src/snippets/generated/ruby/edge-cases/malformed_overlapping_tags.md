---
id: fixture_ruby_malformed_overlapping_tags
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><b><i>bold and italic</b></i></p>')

```
