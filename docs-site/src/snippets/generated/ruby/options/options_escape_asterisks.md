---
id: fixture_ruby_options_escape_asterisks
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Use 2*3 = 6 in math.</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_asterisks: true))

```
