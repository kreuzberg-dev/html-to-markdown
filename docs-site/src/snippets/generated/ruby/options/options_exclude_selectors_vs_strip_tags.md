---
id: fixture_ruby_options_exclude_selectors_vs_strip_tags
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="wrapper"><p>Inner paragraph</p></div><p>Outer text</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.wrapper']))

```
