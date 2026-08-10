```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options_value)

```
