---
id: fixture_python_link_empty_href
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="">No destination</a>'
    _ = convert(html, None)

main()

```
