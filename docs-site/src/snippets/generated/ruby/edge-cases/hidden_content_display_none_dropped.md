---
id: fixture_ruby_hidden_content_display_none_dropped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>visible</p><div style="display:none">secret hidden text</div><p>also visible</p>')

```
