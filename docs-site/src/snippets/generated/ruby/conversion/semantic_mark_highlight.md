---
id: fixture_ruby_semantic_mark_highlight
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>This is <mark>highlighted text</mark> in a sentence.</p>')

```
