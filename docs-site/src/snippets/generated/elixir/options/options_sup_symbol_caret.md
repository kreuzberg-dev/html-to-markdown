```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{sup_symbol: "^"}
result = HtmlToMarkdown.convert("<p>x<sup>2</sup></p>", options_value)

```
