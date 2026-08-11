---
id: fixture_ruby_conversion_autolink_https_url
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="https://example.com">https://example.com</a>')

```
