---
id: fixture_elixir_blockquote_text_then_paragraph_gets_blank_line
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>")

```
