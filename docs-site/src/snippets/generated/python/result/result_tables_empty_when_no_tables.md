---
id: fixture_python_result_tables_empty_when_no_tables
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>No tables here</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
