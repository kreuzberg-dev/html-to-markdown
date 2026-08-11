---
id: fixture_ruby_options_wrap_disabled
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>', HtmlToMarkdownRs::ConversionOptions.new(wrap: false))

```
