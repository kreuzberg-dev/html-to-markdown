---
id: fixture_python_blockquote_simple
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><p>Quote text</p></blockquote>"
    _ = convert(html, None)

main()

```
