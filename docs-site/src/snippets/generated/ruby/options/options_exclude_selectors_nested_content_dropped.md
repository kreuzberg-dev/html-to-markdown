---
id: fixture_ruby_options_exclude_selectors_nested_content_dropped
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><aside class="sidebar"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.sidebar']))

```
