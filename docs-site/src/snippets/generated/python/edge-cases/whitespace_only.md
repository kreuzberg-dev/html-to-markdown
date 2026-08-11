---
id: fixture_python_whitespace_only
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>   </p>"
    _ = convert(html, None)

main()

```
