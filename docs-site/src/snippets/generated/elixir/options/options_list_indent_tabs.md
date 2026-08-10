```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{list_indent_type: "Tabs"}
result = HtmlToMarkdown.convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options_value)

```
