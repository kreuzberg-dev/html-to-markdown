```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{br_in_tables: false}
result = HtmlToMarkdown.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options_value)

```
