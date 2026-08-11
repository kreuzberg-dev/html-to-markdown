---
id: fixture_elixir_malformed_overlapping_tags
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p><b><i>bold and italic</b></i></p>")

```
