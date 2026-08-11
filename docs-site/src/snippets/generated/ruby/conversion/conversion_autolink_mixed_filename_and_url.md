---
id: fixture_ruby_conversion_autolink_mixed_filename_and_url
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>')

```
