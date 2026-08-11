---
id: fixture_ruby_options_br_in_tables_true
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>', HtmlToMarkdownRs::ConversionOptions.new(br_in_tables: true))

```
