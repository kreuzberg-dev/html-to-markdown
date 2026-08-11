---
id: fixture_elixir_options_code_block_backticks
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Backticks"}
result = HtmlToMarkdown.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", options_value)

```
