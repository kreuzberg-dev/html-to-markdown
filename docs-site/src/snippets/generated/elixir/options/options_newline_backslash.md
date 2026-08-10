```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{newline_style: "Backslash"}
result = HtmlToMarkdown.convert("<p>Line one<br>Line two</p>", options_value)

```
