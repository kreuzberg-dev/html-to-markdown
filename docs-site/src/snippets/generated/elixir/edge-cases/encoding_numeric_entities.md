---
id: fixture_elixir_encoding_numeric_entities
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Copyright: &\#169; Trade: &\#174; Euro: &\#8364; Hex: &\#x00A9;</p>")

```
