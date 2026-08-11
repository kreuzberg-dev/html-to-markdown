---
id: fixture_python_emphasis_superscript
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>x<sup>2</sup></p>"
    _ = convert(html, None)

main()

```
