---
id: fixture_ruby_result_warnings_empty_for_malformed_html
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Unclosed paragraph<div>Mixed nesting</p></div>')

```
