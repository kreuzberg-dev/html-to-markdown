---
id: fixture_ruby_image_linked
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="https://example.com"><img src="icon.png" alt="Icon"></a>')

```
