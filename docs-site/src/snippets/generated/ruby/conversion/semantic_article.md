---
id: fixture_ruby_semantic_article
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<article><h2>Article Title</h2><p>Article body.</p></article>')

```
