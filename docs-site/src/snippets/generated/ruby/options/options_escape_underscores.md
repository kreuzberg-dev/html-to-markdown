---
id: fixture_ruby_options_escape_underscores
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>The variable_name is defined.</p>', HtmlToMarkdownRs::ConversionOptions.new(escape_underscores: true))

```
