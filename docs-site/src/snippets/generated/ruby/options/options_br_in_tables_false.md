---
id: fixture_ruby_options_br_in_tables_false
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>', HtmlToMarkdownRs::ConversionOptions.new(br_in_tables: false))

```
