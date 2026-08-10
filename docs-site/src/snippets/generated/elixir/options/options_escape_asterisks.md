```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_asterisks: true}
result = HtmlToMarkdown.convert("<p>Use 2*3 = 6 in math.</p>", options_value)

```
