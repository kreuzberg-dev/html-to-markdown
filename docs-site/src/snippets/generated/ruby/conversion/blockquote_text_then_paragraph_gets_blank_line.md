---
id: fixture_ruby_blockquote_text_then_paragraph_gets_blank_line
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<blockquote>Just text, then <p>a paragraph</p></blockquote>')

```
