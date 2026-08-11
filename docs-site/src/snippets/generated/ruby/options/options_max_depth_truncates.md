---
id: fixture_ruby_options_max_depth_truncates
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>', HtmlToMarkdownRs::ConversionOptions.new(max_depth: 3))

```
