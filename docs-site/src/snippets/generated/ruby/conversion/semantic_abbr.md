---
id: fixture_ruby_semantic_abbr
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>The <abbr title="World Wide Web">WWW</abbr> is global.</p>')

```
