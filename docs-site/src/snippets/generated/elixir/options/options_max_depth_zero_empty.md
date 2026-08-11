---
id: fixture_elixir_options_max_depth_zero_empty
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{max_depth: 0}
result = HtmlToMarkdown.convert("<p>Hello</p>", options_value)

```
