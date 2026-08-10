```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_ascii: true}
result = HtmlToMarkdown.convert("<p>Text with \# hash and [brackets] and * star</p>", options_value)

```
