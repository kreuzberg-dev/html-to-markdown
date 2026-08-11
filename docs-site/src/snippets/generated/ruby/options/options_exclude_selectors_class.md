---
id: fixture_ruby_options_exclude_selectors_class
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.cookie-banner']))

```
