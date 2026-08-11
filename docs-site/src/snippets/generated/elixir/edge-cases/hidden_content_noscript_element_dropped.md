---
id: fixture_elixir_hidden_content_noscript_element_dropped
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>")

```
