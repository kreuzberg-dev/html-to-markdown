---
id: fixture_ruby_options_strip_newlines
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>First paragraph.</p><p>Second paragraph.</p>', HtmlToMarkdownRs::ConversionOptions.new(strip_newlines: true))

```
