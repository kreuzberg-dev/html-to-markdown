---
id: fixture_elixir_table_nested_chain_not_misclassified_as_layout
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>")

```
