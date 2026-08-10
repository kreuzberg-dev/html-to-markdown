```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{sub_symbol: "~"}
result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O</p>", options_value)

```
