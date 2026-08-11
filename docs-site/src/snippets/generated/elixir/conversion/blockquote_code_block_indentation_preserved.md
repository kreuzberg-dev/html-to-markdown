---
id: fixture_elixir_blockquote_code_block_indentation_preserved
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>")

```
