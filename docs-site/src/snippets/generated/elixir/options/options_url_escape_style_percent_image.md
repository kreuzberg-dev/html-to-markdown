---
id: fixture_elixir_options_url_escape_style_percent_image
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "percent"}
result = HtmlToMarkdown.convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options_value)

```
