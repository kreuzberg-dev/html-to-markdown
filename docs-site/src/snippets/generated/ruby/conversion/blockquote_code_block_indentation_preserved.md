---
id: fixture_ruby_blockquote_code_block_indentation_preserved
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>")

```
