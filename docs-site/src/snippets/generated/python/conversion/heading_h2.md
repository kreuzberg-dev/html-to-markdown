---
id: fixture_python_heading_h2
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h2>Heading 2</h2>"
    _ = convert(html, None)

main()

```
