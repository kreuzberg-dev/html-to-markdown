---
id: fixture_python_paragraph_multiple
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>First paragraph.</p><p>Second paragraph.</p>"
    _ = convert(html, None)

main()

```
