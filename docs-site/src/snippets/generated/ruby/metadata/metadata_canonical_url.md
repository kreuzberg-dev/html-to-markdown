---
id: fixture_ruby_metadata_canonical_url
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Page</title><link rel="canonical" href="https://example.com/canonical-page"></head><body><p>Content</p></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
