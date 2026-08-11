---
id: fixture_elixir_style_tags_only
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>")

```
