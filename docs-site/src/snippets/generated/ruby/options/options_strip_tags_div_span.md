---
id: fixture_ruby_options_strip_tags_div_span
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", HtmlToMarkdownRs::ConversionOptions.new(strip_tags: ['div', 'span']))

```
