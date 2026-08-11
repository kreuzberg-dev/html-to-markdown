---
id: fixture_elixir_options_url_escape_style_percent_angle_brackets_in_url
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "percent"}
result = HtmlToMarkdown.convert("<a href=\"/file (1) <draft>.pdf\">file</a>", options_value)

```
