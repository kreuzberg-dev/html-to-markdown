```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", options_value)

```
