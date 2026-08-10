```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_images: true}
result = HtmlToMarkdown.convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options_value)

```
