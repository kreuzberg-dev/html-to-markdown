---
id: fixture_ruby_metadata_keywords_meta
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Page</title><meta name="keywords" content="rust, markdown, html, converter"></head><body><p>Content</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
