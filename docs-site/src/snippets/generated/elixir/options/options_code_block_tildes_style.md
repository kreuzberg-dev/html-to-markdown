---
id: fixture_elixir_options_code_block_tildes_style
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Tildes"}
result = HtmlToMarkdown.convert("<pre><code>some code</code></pre>", options_value)

```
