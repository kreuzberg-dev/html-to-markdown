---
id: fixture_python_semantic_hr
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Above</p><hr><p>Below</p>"
    _ = convert(html, None)

main()

```
