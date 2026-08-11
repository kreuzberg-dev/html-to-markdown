---
id: fixture_elixir_result_warning_kind_image_extraction_failed
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_images: true}
result = HtmlToMarkdown.convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options_value)

```
