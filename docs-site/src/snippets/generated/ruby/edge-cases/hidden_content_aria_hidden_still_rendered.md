---
id: fixture_ruby_hidden_content_aria_hidden_still_rendered
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>visible</p><div aria-hidden="true">still shown</div><p>also visible</p>')

```
