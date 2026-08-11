---
id: fixture_ruby_metadata_link_type_external_classified
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>See <a href="https://example.com">Example</a> for details.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
