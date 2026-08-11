---
id: fixture_elixir_html_comments_only
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<!-- This is a comment --><!-- Another comment -->")

```
