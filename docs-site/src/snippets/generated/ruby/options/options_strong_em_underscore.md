---
id: fixture_ruby_options_strong_em_underscore
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><strong>bold</strong> and <em>italic</em></p>', HtmlToMarkdownRs::ConversionOptions.new(strong_em_symbol: '_'))

```
