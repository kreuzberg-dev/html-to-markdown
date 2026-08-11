---
id: fixture_ruby_structure_sibling_h1_groups
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
