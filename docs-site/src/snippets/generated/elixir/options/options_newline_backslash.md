---
id: fixture_elixir_options_newline_backslash
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{newline_style: "Backslash"}
result = HtmlToMarkdown.convert("<p>Line one<br>Line two</p>", options_value)

```
