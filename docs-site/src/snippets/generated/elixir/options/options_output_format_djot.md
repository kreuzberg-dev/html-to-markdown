---
id: fixture_elixir_options_output_format_djot
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{output_format: "Djot"}
result = HtmlToMarkdown.convert("<p>Simple paragraph.</p>", options_value)

```
