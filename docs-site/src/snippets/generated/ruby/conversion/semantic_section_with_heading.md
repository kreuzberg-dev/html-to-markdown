---
id: fixture_ruby_semantic_section_with_heading
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<section><h3>Section Heading</h3><p>Section content.</p></section>')

```
