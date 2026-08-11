---
id: fixture_ruby_result_tables_empty_when_no_tables
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>No tables here</p>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
