---
id: fixture_elixir_options_strong_em_underscore
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{strong_em_symbol: "_"}
result = HtmlToMarkdown.convert("<p><strong>bold</strong> and <em>italic</em></p>", options_value)

```
