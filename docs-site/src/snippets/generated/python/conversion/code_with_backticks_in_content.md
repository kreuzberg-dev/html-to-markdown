---
id: fixture_python_code_with_backticks_in_content
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Use <code>`backtick` here</code> carefully.</p>"
    _ = convert(html, None)

main()

```
