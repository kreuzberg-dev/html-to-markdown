---
id: fixture_python_table_empty
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table></table>"
    _ = convert(html, None)

main()

```
