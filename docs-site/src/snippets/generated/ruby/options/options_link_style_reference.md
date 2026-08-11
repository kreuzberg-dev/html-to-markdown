---
id: fixture_ruby_options_link_style_reference
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", HtmlToMarkdownRs::ConversionOptions.new(link_style: 'Reference'))

```
