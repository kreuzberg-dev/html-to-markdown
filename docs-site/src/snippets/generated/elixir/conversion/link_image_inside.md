---
id: fixture_elixir_link_image_inside
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>")

```
