---
id: fixture_ruby_options_url_escape_style_percent_link
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="/file (1).pdf">file</a>', HtmlToMarkdownRs::ConversionOptions.new(url_escape_style: 'percent'))

```
