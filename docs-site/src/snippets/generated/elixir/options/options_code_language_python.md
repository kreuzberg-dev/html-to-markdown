---
id: fixture_elixir_options_code_language_python
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_language: "python"}
result = HtmlToMarkdown.convert("<pre><code>def hello(): pass</code></pre>", options_value)

```
