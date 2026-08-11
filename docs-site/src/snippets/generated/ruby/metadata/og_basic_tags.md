---
id: fixture_ruby_og_basic_tags
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"><meta property="og:description" content="OG description text."><meta property="og:image" content="https://example.com/image.jpg"></head><body><p>Content</p></body></html>')

```
