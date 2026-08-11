---
id: fixture_ruby_options_newline_backslash
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Line one<br>Line two</p>', HtmlToMarkdownRs::ConversionOptions.new(newline_style: 'Backslash'))

```
