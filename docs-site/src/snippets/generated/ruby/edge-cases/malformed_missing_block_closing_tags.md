---
id: fixture_ruby_malformed_missing_block_closing_tags
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<div><h1>Title<p>First paragraph<p>Second paragraph</div>')

```
