```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", options_value)

```
