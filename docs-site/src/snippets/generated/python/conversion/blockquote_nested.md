---
id: fixture_python_blockquote_nested
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>"
    _ = convert(html, None)

main()

```
