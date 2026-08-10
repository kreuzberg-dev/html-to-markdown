```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>", options_value)

```
