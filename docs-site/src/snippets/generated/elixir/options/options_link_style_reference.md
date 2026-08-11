---
id: fixture_elixir_options_link_style_reference
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{link_style: "Reference"}
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options_value)

```
