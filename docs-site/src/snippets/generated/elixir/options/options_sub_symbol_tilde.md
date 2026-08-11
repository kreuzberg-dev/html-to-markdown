---
id: fixture_elixir_options_sub_symbol_tilde
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{sub_symbol: "~"}
result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O</p>", options_value)

```
