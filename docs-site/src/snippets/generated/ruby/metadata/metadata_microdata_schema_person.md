---
id: fixture_ruby_metadata_microdata_schema_person
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Contact</title></head><body><div itemscope itemtype="https://schema.org/Person"><span itemprop="name">John Smith</span><span itemprop="email">john@example.com</span><span itemprop="telephone">+1-555-0100</span></div></body></html>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
