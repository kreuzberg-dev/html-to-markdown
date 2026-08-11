---
id: fixture_python_smoke_empty_string
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = ""
    _ = convert(html, None)

main()

```
