---
id: fixture_elixir_encoding_unicode_emoji
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>")

```
