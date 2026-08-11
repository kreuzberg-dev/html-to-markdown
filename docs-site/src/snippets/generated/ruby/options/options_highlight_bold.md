---
id: fixture_ruby_options_highlight_bold
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with <mark>highlighted</mark> text.</p>', HtmlToMarkdownRs::ConversionOptions.new(highlight_style: 'Bold'))

```
