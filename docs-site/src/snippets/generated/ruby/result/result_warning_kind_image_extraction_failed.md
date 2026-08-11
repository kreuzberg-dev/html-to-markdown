---
id: fixture_ruby_result_warning_kind_image_extraction_failed
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Text<img src="data:BADMIME" alt="broken">end</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_images: true))

```
