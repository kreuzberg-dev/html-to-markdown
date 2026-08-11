---
id: fixture_python_image_no_alt
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<img src="banner.jpg">'
    _ = convert(html, None)

main()

```
