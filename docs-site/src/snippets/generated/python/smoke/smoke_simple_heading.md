---
id: fixture_python_smoke_simple_heading
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>Title</h1>"
    _ = convert(html, None)

main()

```
