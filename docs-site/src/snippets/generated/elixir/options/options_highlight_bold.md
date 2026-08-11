---
id: fixture_elixir_options_highlight_bold
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{highlight_style: "Bold"}
result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> text.</p>", options_value)

```
