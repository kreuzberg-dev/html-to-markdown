---
id: fixture_ruby_link_with_title
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="https://example.com" title="Example Site">Example</a>')

```
