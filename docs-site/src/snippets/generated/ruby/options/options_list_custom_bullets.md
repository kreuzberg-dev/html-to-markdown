---
id: fixture_ruby_options_list_custom_bullets
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<ul><li>Item A</li><li>Item B</li></ul>', HtmlToMarkdownRs::ConversionOptions.new(bullets: '*'))

```
