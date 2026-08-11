---
id: fixture_ruby_paragraph_with_inline_formatting
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href="https://example.com">link</a>.</p>')

```
