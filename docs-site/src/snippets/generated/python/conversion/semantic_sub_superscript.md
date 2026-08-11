---
id: fixture_python_semantic_sub_superscript
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>"
    _ = convert(html, None)

main()

```
