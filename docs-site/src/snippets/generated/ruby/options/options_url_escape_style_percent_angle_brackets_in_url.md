---
id: fixture_ruby_options_url_escape_style_percent_angle_brackets_in_url
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="/file (1) <draft>.pdf">file</a>', HtmlToMarkdownRs::ConversionOptions.new(url_escape_style: 'percent'))

```
