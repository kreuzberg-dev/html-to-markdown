---
id: fixture_elixir_blockquote_nested_list_indentation_preserved
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>")

```
