---
id: fixture_ruby_options_highlight_none
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>plain</mark> content.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'None'))

```
