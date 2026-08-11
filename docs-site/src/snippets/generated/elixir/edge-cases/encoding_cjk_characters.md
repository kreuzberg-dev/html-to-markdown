---
id: fixture_elixir_encoding_cjk_characters
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>")

```
