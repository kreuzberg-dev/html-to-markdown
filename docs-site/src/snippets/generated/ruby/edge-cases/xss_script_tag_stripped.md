---
id: fixture_ruby_xss_script_tag_stripped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>")

```
