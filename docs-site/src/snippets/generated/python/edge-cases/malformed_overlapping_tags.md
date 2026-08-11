---
id: fixture_python_malformed_overlapping_tags
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><b><i>bold and italic</b></i></p>"
    _ = convert(html, None)

main()

```
