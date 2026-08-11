---
id: fixture_elixir_options_heading_style_underlined
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{heading_style: "Underlined"}
result = HtmlToMarkdown.convert("<h1>Main Title</h1>", options_value)

```
