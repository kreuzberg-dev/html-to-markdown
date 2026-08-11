---
id: fixture_ruby_form_input_elements
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<form><label for="name">Name:</label><input type="text" id="name" placeholder="Enter name"></form>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_forms' => false }))

```
