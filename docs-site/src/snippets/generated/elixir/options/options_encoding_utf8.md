---
id: fixture_elixir_options_encoding_utf8
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{encoding: "utf-8"}
result = HtmlToMarkdown.convert("<p>Café naïve résumé</p>", options_value)

```
