```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{list_indent_width: 4}
result = HtmlToMarkdown.convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options_value)

```
