---
id: fixture_ruby_script_tags_only
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>")

```
