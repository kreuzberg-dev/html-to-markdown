---
id: fixture_python_emphasis_subscript
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>H<sub>2</sub>O</p>"
    _ = convert(html, None)

main()

```
