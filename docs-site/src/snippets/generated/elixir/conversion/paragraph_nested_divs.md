---
id: fixture_elixir_paragraph_nested_divs
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<div><div><p>Nested text</p></div></div>")

```
