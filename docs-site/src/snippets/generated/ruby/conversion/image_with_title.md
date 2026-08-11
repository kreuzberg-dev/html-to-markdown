---
id: fixture_ruby_image_with_title
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<img src="chart.png" alt="Sales chart" title="Q3 Sales">')

```
