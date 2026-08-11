---
id: fixture_ruby_bold_and_italic
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><strong><em>both</em></strong></p>')

```
