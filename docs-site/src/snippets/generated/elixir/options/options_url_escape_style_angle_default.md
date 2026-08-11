---
id: fixture_elixir_options_url_escape_style_angle_default
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "angle"}
result = HtmlToMarkdown.convert("<a href=\"/file (1).pdf\">file</a>", options_value)

```
