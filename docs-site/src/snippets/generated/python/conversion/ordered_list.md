---
id: fixture_python_ordered_list
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ol><li>First</li><li>Second</li><li>Third</li></ol>"
    _ = convert(html, None)

main()

```
