```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options_value)

```
