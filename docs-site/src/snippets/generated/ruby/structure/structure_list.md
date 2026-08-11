---
id: fixture_ruby_structure_list
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
