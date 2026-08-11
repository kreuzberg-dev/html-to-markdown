---
id: fixture_ruby_hidden_content_noscript_element_dropped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>')

```
