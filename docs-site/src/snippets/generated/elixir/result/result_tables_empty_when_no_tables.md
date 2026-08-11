---
id: fixture_elixir_result_tables_empty_when_no_tables
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<p>No tables here</p>", options_value)

```
