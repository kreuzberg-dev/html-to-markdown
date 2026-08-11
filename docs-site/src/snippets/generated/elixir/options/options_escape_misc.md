---
id: fixture_elixir_options_escape_misc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_misc: true}
result = HtmlToMarkdown.convert("<p>Use \# and | and ~ in text.</p>", options_value)

```
