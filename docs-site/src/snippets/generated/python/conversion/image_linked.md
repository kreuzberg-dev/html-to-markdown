---
id: fixture_python_image_linked
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="https://example.com"><img src="icon.png" alt="Icon"></a>'
    _ = convert(html, None)

main()

```
