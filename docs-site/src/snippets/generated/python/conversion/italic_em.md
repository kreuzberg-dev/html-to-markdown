---
id: fixture_python_italic_em
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><em>italic</em></p>"
    _ = convert(html, None)

main()

```
