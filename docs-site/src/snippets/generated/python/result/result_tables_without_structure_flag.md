---
id: fixture_python_result_tables_without_structure_flag
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>"
    _ = convert(html, None)

main()

```
