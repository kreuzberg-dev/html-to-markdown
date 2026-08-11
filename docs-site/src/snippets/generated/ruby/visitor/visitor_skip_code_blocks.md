---
id: fixture_ruby_visitor_skip_code_blocks
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_code_block(ctx, lang, code)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>', visitor)

```
