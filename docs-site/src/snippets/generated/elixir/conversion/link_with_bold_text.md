---
id: fixture_elixir_link_with_bold_text
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>")

```
