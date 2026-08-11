---
id: fixture_elixir_options_code_block_tildes
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Tildes"}
result = HtmlToMarkdown.convert("<pre><code>let x = 1;</code></pre>", options_value)

```
