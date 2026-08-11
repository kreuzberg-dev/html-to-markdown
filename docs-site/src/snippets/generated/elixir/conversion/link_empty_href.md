---
id: fixture_elixir_link_empty_href
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<a href=\"\">No destination</a>")

```
