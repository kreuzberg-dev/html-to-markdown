---
id: fixture_ruby_options_highlight_double_equal
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>highlighted</mark> here.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'DoubleEqual'))

```
