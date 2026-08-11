---
id: fixture_ruby_options_debug_true
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Debug test</p>', HtmlToMarkdownRs::ConversionOptions.new(debug: true))

```
