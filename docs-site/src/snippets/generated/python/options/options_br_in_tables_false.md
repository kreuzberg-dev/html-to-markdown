---
id: fixture_python_options_br_in_tables_false
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>"
    options = ConversionOptions(br_in_tables=False)
    _ = convert(html, options)

main()

```
