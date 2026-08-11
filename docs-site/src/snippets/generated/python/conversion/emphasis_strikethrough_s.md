---
id: fixture_python_emphasis_strikethrough_s
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><s>strikethrough</s></p>"
    _ = convert(html, None)

main()

```
