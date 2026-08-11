---
id: fixture_ruby_options_heading_style_atx
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Title</h1><h2>Subtitle</h2>', HtmlToMarkdownRs::ConversionOptions.new(heading_style: 'Atx'))

```
