---
id: fixture_ruby_encoding_unicode_emoji
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>')

```
