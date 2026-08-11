---
id: fixture_python_heading_h3
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h3>Heading 3</h3>"
    _ = convert(html, None)

main()

```
