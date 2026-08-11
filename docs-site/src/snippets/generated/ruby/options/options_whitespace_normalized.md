---
id: fixture_ruby_options_whitespace_normalized
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text   with    extra   spaces.</p>', HtmlToMarkdownRs::ConversionOptions.new(whitespace_mode: 'Normalized'))

```
