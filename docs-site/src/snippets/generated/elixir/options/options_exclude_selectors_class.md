---
id: fixture_elixir_options_exclude_selectors_class
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".cookie-banner"]}
result = HtmlToMarkdown.convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options_value)

```
