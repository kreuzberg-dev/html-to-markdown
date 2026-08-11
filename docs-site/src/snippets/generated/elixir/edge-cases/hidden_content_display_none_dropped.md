---
id: fixture_elixir_hidden_content_display_none_dropped
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>")

```
