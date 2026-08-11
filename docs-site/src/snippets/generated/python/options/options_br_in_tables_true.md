---
id: fixture_python_options_br_in_tables_true
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>"
    options = ConversionOptions(br_in_tables=True)
    _ = convert(html, options)

main()

```
