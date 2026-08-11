---
id: fixture_python_hidden_content_aria_hidden_still_rendered
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<p>visible</p><div aria-hidden="true">still shown</div><p>also visible</p>'
    _ = convert(html, None)

main()

```
