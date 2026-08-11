---
id: fixture_elixir_hidden_content_aria_hidden_still_rendered
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>")

```
