---
id: fixture_elixir_options_convert_as_inline
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{convert_as_inline: true}
result = HtmlToMarkdown.convert("<p>One</p><p>Two</p>", options_value)

```
