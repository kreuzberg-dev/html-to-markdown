```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{max_depth: 3}
result = HtmlToMarkdown.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options_value)

```
