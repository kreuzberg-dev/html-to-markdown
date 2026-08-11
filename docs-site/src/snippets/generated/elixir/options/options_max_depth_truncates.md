---
id: fixture_elixir_options_max_depth_truncates
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{max_depth: 3}
result = HtmlToMarkdown.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options_value)

```
