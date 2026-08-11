---
id: fixture_ruby_options_keep_inline_images_in_paragraph
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", HtmlToMarkdownRs::ConversionOptions.new(keep_inline_images_in: ['p']))

```
