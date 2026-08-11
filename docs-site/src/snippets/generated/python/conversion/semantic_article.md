---
id: fixture_python_semantic_article
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<article><h2>Article Title</h2><p>Article body.</p></article>"
    _ = convert(html, None)

main()

```
