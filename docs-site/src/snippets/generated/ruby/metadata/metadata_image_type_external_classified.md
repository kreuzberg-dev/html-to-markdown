---
id: fixture_ruby_metadata_image_type_external_classified
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p><img src="https://example.com/photo.jpg" alt="A photo"></p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
