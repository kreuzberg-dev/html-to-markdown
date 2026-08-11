---
id: fixture_python_heading_h4
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h4>Heading 4</h4>"
    _ = convert(html, None)

main()

```
