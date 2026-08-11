---
id: fixture_elixir_options_include_document_structure_false
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: false}
result = HtmlToMarkdown.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options_value)

```
