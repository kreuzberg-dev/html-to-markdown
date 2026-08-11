---
id: fixture_ruby_link_with_bold_text
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="https://example.com"><strong>Bold link</strong></a>')

```
