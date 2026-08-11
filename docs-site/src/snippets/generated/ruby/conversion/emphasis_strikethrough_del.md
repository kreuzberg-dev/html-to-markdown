---
id: fixture_ruby_emphasis_strikethrough_del
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><del>deleted text</del></p>')

```
