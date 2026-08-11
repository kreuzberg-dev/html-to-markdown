---
id: fixture_elixir_semantic_sub_superscript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>")

```
