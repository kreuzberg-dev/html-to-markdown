---
id: fixture_elixir_options_escape_asterisks
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_asterisks: true}
result = HtmlToMarkdown.convert("<p>Use 2*3 = 6 in math.</p>", options_value)

```
