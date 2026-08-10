```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{highlight_style: "None"}
result = HtmlToMarkdown.convert("<p>Text with <mark>plain</mark> content.</p>", options_value)

```
