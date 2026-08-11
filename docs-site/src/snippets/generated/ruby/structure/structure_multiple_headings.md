---
id: fixture_ruby_structure_multiple_headings
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
