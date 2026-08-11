---
id: fixture_ruby_encoding_cjk_characters
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>')

```
