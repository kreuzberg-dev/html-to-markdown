---
id: fixture_elixir_options_default_title_true
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{default_title: true}
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Link</a></p>", options_value)

```
