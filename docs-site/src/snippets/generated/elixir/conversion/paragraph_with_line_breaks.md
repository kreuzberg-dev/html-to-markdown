---
id: fixture_elixir_paragraph_with_line_breaks
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Line one.<br>Line two.<br>Line three.</p>")

```
