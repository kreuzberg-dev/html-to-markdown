---
id: fixture_ruby_options_default_title_true
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Link</a></p>", HtmlToMarkdownRs::ConversionOptions.new(default_title: true))

```
