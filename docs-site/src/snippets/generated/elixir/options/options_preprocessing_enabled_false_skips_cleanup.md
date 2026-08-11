---
id: fixture_elixir_options_preprocessing_enabled_false_skips_cleanup
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"enabled" => false}}
result = HtmlToMarkdown.convert("<nav>NavSection</nav><p>Paragraph</p>", options_value)

```
