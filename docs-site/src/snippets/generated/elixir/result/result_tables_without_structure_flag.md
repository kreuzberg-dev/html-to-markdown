---
id: fixture_elixir_result_tables_without_structure_flag
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>")

```
