---
id: fixture_ruby_structured_data_json_ld
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Article</title><script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","headline":"My Article","author":{"@type":"Person","name":"Jane Doe"},"datePublished":"2024-01-15"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>')

```
