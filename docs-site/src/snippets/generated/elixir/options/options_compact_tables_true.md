```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{compact_tables: true}
result = HtmlToMarkdown.convert("<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>", options_value)

```
