```python
from html_to_markdown import ConversionOptions, convert

html = """
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"""

# `tables` is collected alongside the document tree, so it must be enabled.
result = convert(html, ConversionOptions(include_document_structure=True))

for table in result.tables:
    for cell in table.grid.cells:
        prefix = "Header" if cell.is_header else "Cell"
        print(f"  {prefix} (r{cell.row},c{cell.col}): {cell.content}")
```
