---
id: fixture_ruby_options_exclude_selectors_plain_text_mode
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="nav">Navigation</div><p>Article body</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.nav'], output_format: 'Plain'))

```
