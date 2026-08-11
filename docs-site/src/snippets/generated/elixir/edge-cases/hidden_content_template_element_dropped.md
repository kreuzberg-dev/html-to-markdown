---
id: fixture_elixir_hidden_content_template_element_dropped
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>visible</p><template><p>secret template text</p></template><p>also visible</p>")

```
