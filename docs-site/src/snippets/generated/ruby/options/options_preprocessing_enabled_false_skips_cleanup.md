---
id: fixture_ruby_options_preprocessing_enabled_false_skips_cleanup
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>NavSection</nav><p>Paragraph</p>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'enabled' => false }))

```
