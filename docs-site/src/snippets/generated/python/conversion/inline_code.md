---
id: fixture_python_inline_code
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Use <code>console.log()</code> to debug</p>"
    _ = convert(html, None)

main()

```
