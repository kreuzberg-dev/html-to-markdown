---
id: fixture_elixir_options_code_block_indented
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Indented"}
result = HtmlToMarkdown.convert("<pre><code>print('hello')</code></pre>", options_value)

```
