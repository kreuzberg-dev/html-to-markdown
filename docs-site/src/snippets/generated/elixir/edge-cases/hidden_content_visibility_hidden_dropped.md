---
id: fixture_elixir_hidden_content_visibility_hidden_dropped
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>")

```
