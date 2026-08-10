```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{bullets: "*"}
result = HtmlToMarkdown.convert("<ul><li>Item A</li><li>Item B</li></ul>", options_value)

```
