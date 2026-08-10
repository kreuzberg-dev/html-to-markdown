```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options_value)

```
