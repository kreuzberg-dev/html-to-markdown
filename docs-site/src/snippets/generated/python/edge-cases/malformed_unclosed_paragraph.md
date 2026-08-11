---
id: fixture_python_malformed_unclosed_paragraph
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>This paragraph is never closed"
    _ = convert(html, None)

main()

```
