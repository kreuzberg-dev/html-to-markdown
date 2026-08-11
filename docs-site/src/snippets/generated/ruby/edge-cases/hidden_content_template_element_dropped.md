---
id: fixture_ruby_hidden_content_template_element_dropped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>visible</p><template><p>secret template text</p></template><p>also visible</p>')

```
