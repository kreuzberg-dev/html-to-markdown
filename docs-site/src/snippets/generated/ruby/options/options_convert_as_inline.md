---
id: fixture_ruby_options_convert_as_inline
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>One</p><p>Two</p>', HtmlToMarkdownRs::ConversionOptions.new(convert_as_inline: true))

```
