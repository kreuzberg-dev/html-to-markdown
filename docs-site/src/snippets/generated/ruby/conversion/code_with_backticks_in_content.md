---
id: fixture_ruby_code_with_backticks_in_content
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Use <code>`backtick` here</code> carefully.</p>')

```
