---
id: fixture_ruby_options_output_format_djot
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Simple paragraph.</p>', HtmlToMarkdownRs::ConversionOptions.new(output_format: 'Djot'))

```
