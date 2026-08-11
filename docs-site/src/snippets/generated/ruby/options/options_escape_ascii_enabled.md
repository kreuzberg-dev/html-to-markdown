---
id: fixture_ruby_options_escape_ascii_enabled
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text with # hash and [brackets] and * star</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_ascii: true))

```
