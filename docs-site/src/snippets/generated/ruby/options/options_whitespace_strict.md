---
id: fixture_ruby_options_whitespace_strict
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Preserved   spacing.</p>', HtmlToMarkdownRs::ConversionOptions.new(whitespace_mode: 'Strict'))

```
