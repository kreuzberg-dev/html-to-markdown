---
id: fixture_ruby_metadata_link_type_anchor_classified
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Jump to <a href="#section">section</a> below.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
