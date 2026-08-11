---
id: fixture_python_heading_h5
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h5>Heading 5</h5>"
    _ = convert(html, None)

main()

```
