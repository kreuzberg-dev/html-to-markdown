---
id: fixture_elixir_options_keep_inline_images_in_paragraph
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{keep_inline_images_in: ["p"]}
result = HtmlToMarkdown.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options_value)

```
