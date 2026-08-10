```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", options_value)

```
