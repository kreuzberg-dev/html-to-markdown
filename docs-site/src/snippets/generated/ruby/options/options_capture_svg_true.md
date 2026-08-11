---
id: fixture_ruby_options_capture_svg_true
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Below SVG:</p><svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"/></svg>', HtmlToMarkdownRs::ConversionOptions.new(capture_svg: true, extract_images: true))

```
