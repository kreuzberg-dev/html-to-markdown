---
id: fixture_ruby_options_output_format_plain
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Title</h1><p>Some <strong>bold</strong> text.</p>', HtmlToMarkdownRs::ConversionOptions.new(output_format: 'Plain'))

```
