---
id: fixture_elixir_result_warnings_empty_for_malformed_html
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>")

```
