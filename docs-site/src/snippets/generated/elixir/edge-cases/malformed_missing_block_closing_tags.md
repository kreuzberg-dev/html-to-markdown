---
id: fixture_elixir_malformed_missing_block_closing_tags
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>")

```
