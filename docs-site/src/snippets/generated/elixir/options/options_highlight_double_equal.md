```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{highlight_style: "DoubleEqual"}
result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> here.</p>", options_value)

```
