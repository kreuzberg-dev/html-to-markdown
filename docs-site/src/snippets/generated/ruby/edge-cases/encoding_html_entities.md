---
id: fixture_ruby_encoding_html_entities
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>')

```
