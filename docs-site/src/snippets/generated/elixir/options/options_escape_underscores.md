---
id: fixture_elixir_options_escape_underscores
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_underscores: true}
result = HtmlToMarkdown.convert("<p>The variable_name is defined.</p>", options_value)

```
