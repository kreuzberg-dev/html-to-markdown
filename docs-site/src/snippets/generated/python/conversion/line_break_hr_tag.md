---
id: fixture_python_line_break_hr_tag
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Before rule.</p><hr><p>After rule.</p>"
    _ = convert(html, None)

main()

```
