---
id: fixture_ruby_options_url_escape_style_percent_image
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<img src="/img (1) <draft>.png" alt="alt">', HtmlToMarkdownRs::ConversionOptions.new(url_escape_style: 'percent'))

```
