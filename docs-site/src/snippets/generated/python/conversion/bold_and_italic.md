---
id: fixture_python_bold_and_italic
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><strong><em>both</em></strong></p>"
    _ = convert(html, None)

main()

```
