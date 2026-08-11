---
id: fixture_elixir_options_exclude_selectors_plain_text_mode
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".nav"], output_format: "Plain"}
result = HtmlToMarkdown.convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options_value)

```
