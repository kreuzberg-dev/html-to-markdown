---
id: fixture_elixir_options_exclude_selectors_empty_noop
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: []}
result = HtmlToMarkdown.convert("<p>Hello world</p>", options_value)

```
