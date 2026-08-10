```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_misc: true}
result = HtmlToMarkdown.convert("<p>Use \# and | and ~ in text.</p>", options_value)

```
