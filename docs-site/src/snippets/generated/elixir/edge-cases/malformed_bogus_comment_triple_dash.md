---
id: fixture_elixir_malformed_bogus_comment_triple_dash
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>")

```
