---
id: fixture_ruby_result_warnings_empty_for_clean_input
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>")

```
