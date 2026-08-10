```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html lang=\"es\"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>", options_value)

```
