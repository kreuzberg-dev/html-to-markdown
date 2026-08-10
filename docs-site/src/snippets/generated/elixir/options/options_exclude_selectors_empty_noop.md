```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: []}
result = HtmlToMarkdown.convert("<p>Hello world</p>", options_value)

```
