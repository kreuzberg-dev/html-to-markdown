---
id: fixture_elixir_options_output_format_markdown
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{heading_style: "Atx", output_format: "Markdown"}
result = HtmlToMarkdown.convert("<h1>Title</h1><p>Some text.</p>", options_value)

```
