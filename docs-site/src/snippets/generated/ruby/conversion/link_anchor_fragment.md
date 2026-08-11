---
id: fixture_ruby_link_anchor_fragment
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="#section">Jump to section</a>')

```
