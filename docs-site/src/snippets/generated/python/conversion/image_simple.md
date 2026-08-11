---
id: fixture_python_image_simple
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<img src="photo.jpg" alt="A photo">'
    _ = convert(html, None)

main()

```
