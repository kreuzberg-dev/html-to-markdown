---
id: fixture_elixir_options_highlight_double_equal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{highlight_style: "DoubleEqual"}
result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> here.</p>", options_value)

```
