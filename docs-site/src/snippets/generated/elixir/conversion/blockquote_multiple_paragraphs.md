---
id: fixture_elixir_blockquote_multiple_paragraphs
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>")

```
