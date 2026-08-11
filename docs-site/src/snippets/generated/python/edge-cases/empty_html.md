---
id: fixture_python_empty_html
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<html><head></head><body></body></html>"
    _ = convert(html, None)

main()

```
