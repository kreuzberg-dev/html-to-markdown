---
id: fixture_python_emphasis_mark_highlight
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><mark>highlighted</mark></p>"
    _ = convert(html, None)

main()

```
