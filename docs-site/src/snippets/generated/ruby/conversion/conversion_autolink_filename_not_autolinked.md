---
id: fixture_ruby_conversion_autolink_filename_not_autolinked
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="foobar.png">foobar.png</a>')

```
