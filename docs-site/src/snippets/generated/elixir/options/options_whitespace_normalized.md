---
id: fixture_elixir_options_whitespace_normalized
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{whitespace_mode: "Normalized"}
result = HtmlToMarkdown.convert("<p>Text   with    extra   spaces.</p>", options_value)

```
