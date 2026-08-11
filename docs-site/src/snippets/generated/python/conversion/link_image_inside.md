---
id: fixture_python_link_image_inside
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="https://example.com"><img src="logo.png" alt="Logo"></a>'
    _ = convert(html, None)

main()

```
