---
id: fixture_elixir_options_debug_true
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{debug: true}
result = HtmlToMarkdown.convert("<p>Debug test</p>", options_value)

```
