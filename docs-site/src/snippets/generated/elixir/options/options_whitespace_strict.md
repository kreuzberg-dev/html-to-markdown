---
id: fixture_elixir_options_whitespace_strict
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{whitespace_mode: "Strict"}
result = HtmlToMarkdown.convert("<p>Preserved   spacing.</p>", options_value)

```
