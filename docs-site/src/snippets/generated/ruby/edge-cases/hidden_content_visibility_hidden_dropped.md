---
id: fixture_ruby_hidden_content_visibility_hidden_dropped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>visible</p><span style="visibility:hidden">secret hidden span</span><p>also visible</p>')

```
