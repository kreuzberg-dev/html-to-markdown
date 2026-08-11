---
id: fixture_ruby_options_sup_symbol_caret
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>x<sup>2</sup></p>', HtmlToMarkdownRs::ConversionOptions.new(sup_symbol: '^'))

```
