```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{br_in_tables: true}
result = HtmlToMarkdown.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options_value)

```
