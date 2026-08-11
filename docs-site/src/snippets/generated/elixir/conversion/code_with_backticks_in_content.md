---
id: fixture_elixir_code_with_backticks_in_content
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Use <code>`backtick` here</code> carefully.</p>")

```
