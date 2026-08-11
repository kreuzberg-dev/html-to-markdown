---
id: fixture_python_html_comments_only
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<!-- This is a comment --><!-- Another comment -->"
    _ = convert(html, None)

main()

```
