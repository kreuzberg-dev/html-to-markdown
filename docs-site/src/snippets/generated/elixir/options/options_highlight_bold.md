```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{highlight_style: "Bold"}
result = HtmlToMarkdown.convert("<p>Text with <mark>highlighted</mark> text.</p>", options_value)

```
