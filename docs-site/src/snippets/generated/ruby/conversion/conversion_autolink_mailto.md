---
id: fixture_ruby_conversion_autolink_mailto
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="mailto:a@b.com">a@b.com</a>')

```
