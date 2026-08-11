---
id: fixture_ruby_metadata_text_direction_rtl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html lang="ar" dir="rtl"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
