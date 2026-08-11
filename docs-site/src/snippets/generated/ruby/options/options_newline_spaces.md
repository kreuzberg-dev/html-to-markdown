---
id: fixture_ruby_options_newline_spaces
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>First<br>Second</p>', HtmlToMarkdownRs::ConversionOptions.new(newline_style: 'Spaces'))

```
