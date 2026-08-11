---
id: fixture_ruby_options_preprocessing_minimal
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>Navigation</nav><p>Content</p><footer>Footer</footer>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'preset' => 'Minimal' }))

```
