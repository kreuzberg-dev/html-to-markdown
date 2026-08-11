---
id: fixture_ruby_options_max_depth_default_unlimited
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<div><div><div><div><p>Deep content</p></div></div></div></div>')

```
