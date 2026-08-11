---
id: fixture_ruby_options_exclude_selectors_empty_noop
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Hello world</p>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: []))

```
