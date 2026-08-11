---
id: fixture_elixir_options_wrap_enabled
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{wrap: true, wrap_width: 40}
result = HtmlToMarkdown.convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options_value)

```
