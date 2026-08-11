---
id: fixture_elixir_issue_396_backticks_blank_line_after_fence
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Backticks"}
result = HtmlToMarkdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options_value)

```
