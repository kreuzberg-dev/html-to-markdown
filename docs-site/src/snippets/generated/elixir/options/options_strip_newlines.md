---
id: fixture_elixir_options_strip_newlines
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{strip_newlines: true}
result = HtmlToMarkdown.convert("<p>First paragraph.</p><p>Second paragraph.</p>", options_value)

```
