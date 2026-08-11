---
id: fixture_elixir_options_exclude_selectors_id
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: ["\#ad-container"]}
result = HtmlToMarkdown.convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options_value)

```
