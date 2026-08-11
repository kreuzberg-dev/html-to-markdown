---
id: fixture_python_line_break_br_tag
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>First line.<br>Second line.</p>"
    _ = convert(html, None)

main()

```
