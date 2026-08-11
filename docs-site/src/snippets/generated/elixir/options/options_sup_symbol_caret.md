---
id: fixture_elixir_options_sup_symbol_caret
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{sup_symbol: "^"}
result = HtmlToMarkdown.convert("<p>x<sup>2</sup></p>", options_value)

```
