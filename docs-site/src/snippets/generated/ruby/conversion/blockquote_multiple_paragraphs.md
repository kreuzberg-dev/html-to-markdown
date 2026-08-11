---
id: fixture_ruby_blockquote_multiple_paragraphs
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>')

```
