---
id: fixture_ruby_encoding_numeric_entities
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>')

```
