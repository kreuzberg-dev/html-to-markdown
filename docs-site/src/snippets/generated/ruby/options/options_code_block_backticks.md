---
id: fixture_ruby_options_code_block_backticks
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Backticks'))

```
