---
id: fixture_python_link_with_bold_text
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="https://example.com"><strong>Bold link</strong></a>'
    _ = convert(html, None)

main()

```
