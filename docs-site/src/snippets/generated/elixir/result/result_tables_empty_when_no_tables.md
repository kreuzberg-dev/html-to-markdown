```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<p>No tables here</p>", options_value)

```
