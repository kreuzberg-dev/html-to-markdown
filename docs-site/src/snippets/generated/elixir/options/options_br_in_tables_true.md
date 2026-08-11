---
id: fixture_elixir_options_br_in_tables_true
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{br_in_tables: true}
result = HtmlToMarkdown.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options_value)

```
