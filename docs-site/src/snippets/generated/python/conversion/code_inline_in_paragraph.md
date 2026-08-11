---
id: fixture_python_code_inline_in_paragraph
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Call the <code>initialize()</code> method first.</p>"
    _ = convert(html, None)

main()

```
