```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{wrap: false}
result = HtmlToMarkdown.convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options_value)

```
