```ruby
require 'html_to_markdown'

html = <<~HTML
  <table>
      <tr><th>Name</th><th>Age</th></tr>
      <tr><td>Alice</td><td>30</td></tr>
      <tr><td>Bob</td><td>25</td></tr>
  </table>
HTML

# Tables are only populated when `include_document_structure` is enabled.
result = HtmlToMarkdown.convert(html, include_document_structure: true)

result.tables.each do |table|
  table.grid.cells.group_by(&:row).each do |_row, cells|
    prefix = cells.first.is_header ? "Header" : "Row"
    puts "  #{prefix}: #{cells.map(&:content).join(', ')}"
  end
end
```
