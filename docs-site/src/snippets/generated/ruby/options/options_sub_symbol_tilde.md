---
id: fixture_ruby_options_sub_symbol_tilde
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>H<sub>2</sub>O</p>', HtmlToMarkdownRs::ConversionOptions.new(sub_symbol: '~'))

```
