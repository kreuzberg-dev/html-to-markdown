---
id: fixture_ruby_semantic_details_summary
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<details><summary>Click to expand</summary><p>Hidden content here.</p></details>')

```
