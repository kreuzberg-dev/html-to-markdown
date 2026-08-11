---
id: fixture_python_heading_h1
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>Heading 1</h1>"
    _ = convert(html, None)

main()

```
