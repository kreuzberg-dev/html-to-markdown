---
id: fixture_ruby_metadata_extract_all_images
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Gallery</title></head><body><img src="https://example.com/photo1.jpg" alt="Photo 1"><img src="https://example.com/photo2.png" alt="Photo 2"><img src="/local/image.webp" alt="Local image"></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
