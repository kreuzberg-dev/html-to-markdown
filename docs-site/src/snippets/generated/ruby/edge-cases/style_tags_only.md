---
id: fixture_ruby_style_tags_only
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>')

```
