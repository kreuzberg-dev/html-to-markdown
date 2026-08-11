---
id: fixture_ruby_malformed_bogus_comment_triple_dash
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>")

```
