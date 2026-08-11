---
id: fixture_python_result_tables_multiple
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
