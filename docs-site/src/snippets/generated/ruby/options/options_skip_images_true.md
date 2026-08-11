---
id: fixture_ruby_options_skip_images_true
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", HtmlToMarkdownRs::ConversionOptions.new(skip_images: true))

```
