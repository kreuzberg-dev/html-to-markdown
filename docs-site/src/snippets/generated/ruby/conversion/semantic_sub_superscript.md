---
id: fixture_ruby_semantic_sub_superscript
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>')

```
