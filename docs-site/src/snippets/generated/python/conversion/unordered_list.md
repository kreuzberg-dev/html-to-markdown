---
id: fixture_python_unordered_list
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
    _ = convert(html, None)

main()

```
