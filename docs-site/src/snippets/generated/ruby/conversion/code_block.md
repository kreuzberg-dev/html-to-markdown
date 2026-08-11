---
id: fixture_ruby_code_block
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<pre><code class=\"language-python\">print('hello')</code></pre>")

```
