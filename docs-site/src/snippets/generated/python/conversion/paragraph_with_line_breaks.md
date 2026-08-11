---
id: fixture_python_paragraph_with_line_breaks
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Line one.<br>Line two.<br>Line three.</p>"
    _ = convert(html, None)

main()

```
