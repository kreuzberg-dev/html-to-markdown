---
id: fixture_python_smoke_simple_paragraph
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Hello World</p>"
    _ = convert(html, None)

main()

```
