---
id: fixture_ruby_issue_396_backticks_blank_line_after_fence
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Backticks'))

```
