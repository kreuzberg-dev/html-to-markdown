---
id: fixture_ruby_options_output_format_markdown
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<h1>Title</h1><p>Some text.</p>', HtmlToMarkdownRs::ConversionOptions.new(heading_style: 'Atx', output_format: 'Markdown'))

```
