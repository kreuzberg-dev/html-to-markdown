---
id: fixture_ruby_options_encoding_utf8
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Café naïve résumé</p>', HtmlToMarkdownRs::ConversionOptions.new(encoding: 'utf-8'))

```
