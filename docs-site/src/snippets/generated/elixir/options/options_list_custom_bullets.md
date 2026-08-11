---
id: fixture_elixir_options_list_custom_bullets
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{bullets: "*"}
result = HtmlToMarkdown.convert("<ul><li>Item A</li><li>Item B</li></ul>", options_value)

```
