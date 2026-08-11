---
id: fixture_ruby_blockquote_simple
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<blockquote><p>Quote text</p></blockquote>')

```
