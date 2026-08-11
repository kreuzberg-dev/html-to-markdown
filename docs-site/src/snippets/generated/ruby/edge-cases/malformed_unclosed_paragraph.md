---
id: fixture_ruby_malformed_unclosed_paragraph
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>This paragraph is never closed')

```
