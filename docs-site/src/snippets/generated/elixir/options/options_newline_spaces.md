---
id: fixture_elixir_options_newline_spaces
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{newline_style: "Spaces"}
result = HtmlToMarkdown.convert("<p>First<br>Second</p>", options_value)

```
