---
id: fixture_elixir_structure_heading_paragraph
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<h1>Title</h1><p>A paragraph of text.</p>", options_value)

```
