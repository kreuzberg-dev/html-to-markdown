---
id: fixture_python_emphasis_underline_u
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><u>underlined</u></p>"
    _ = convert(html, None)

main()

```
