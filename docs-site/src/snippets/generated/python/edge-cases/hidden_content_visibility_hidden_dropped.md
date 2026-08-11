---
id: fixture_python_hidden_content_visibility_hidden_dropped
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<p>visible</p><span style="visibility:hidden">secret hidden span</span><p>also visible</p>'
    _ = convert(html, None)

main()

```
