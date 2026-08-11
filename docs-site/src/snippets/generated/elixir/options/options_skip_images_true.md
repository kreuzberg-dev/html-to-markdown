---
id: fixture_elixir_options_skip_images_true
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{skip_images: true}
result = HtmlToMarkdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", options_value)

```
