---
id: fixture_ruby_options_escape_misc
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Use # and | and ~ in text.</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_misc: true))

```
