---
id: fixture_ruby_options_exclude_selectors_attribute
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div role="complementary">Sidebar</div><p>Primary text</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ["[role='complementary']"]))

```
