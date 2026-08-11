---
id: fixture_ruby_options_preserve_tags_iframe
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", HtmlToMarkdownRs::ConversionOptions.new(preserve_tags: ['iframe']))

```
