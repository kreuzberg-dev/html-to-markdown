---
id: fixture_ruby_options_heading_style_atx_closed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Closed Heading</h1>', HtmlToMarkdownRs::ConversionOptions.new(heading_style: 'AtxClosed'))

```
