---
id: fixture_elixir_options_heading_style_atx_closed
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{heading_style: "AtxClosed"}
result = HtmlToMarkdown.convert("<h1>Closed Heading</h1>", options_value)

```
