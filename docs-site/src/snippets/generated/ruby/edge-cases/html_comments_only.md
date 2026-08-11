---
id: fixture_ruby_html_comments_only
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<!-- This is a comment --><!-- Another comment -->')

```
