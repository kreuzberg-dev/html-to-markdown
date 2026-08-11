---
id: fixture_elixir_malformed_unclosed_paragraph
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>This paragraph is never closed")

```
