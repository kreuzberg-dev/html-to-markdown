---
id: fixture_ruby_metadata_text_direction_ltr
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
