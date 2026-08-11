---
id: fixture_ruby_blockquote_nested
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>')

```
