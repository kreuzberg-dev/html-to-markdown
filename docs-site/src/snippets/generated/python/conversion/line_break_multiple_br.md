---
id: fixture_python_line_break_multiple_br
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Start.<br><br>End.</p>"
    _ = convert(html, None)

main()

```
