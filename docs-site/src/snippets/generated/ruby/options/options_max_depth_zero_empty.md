---
id: fixture_ruby_options_max_depth_zero_empty
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Hello</p>', HtmlToMarkdownRs::ConversionOptions.new(max_depth: 0))

```
