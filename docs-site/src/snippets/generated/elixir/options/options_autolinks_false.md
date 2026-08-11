---
id: fixture_elixir_options_autolinks_false
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{autolinks: false}
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>https://example.com</a></p>", options_value)

```
