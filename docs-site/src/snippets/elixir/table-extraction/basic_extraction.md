```elixir
html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

opts = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
{:ok, result} = HtmlToMarkdown.convert(html, opts)

for %HtmlToMarkdown.TableData{grid: grid} <- result.tables do
  grid.cells
  |> Enum.group_by(& &1.row)
  |> Enum.sort_by(fn {row, _cells} -> row end)
  |> Enum.each(fn {_row, cells} ->
    cells = Enum.sort_by(cells, & &1.col)
    prefix = if hd(cells).is_header, do: "Header", else: "Row"
    values = Enum.map(cells, & &1.content)
    IO.puts("  #{prefix}: #{Enum.join(values, ", ")}")
  end)
end
```
