---
id: fixture_python_hidden_content_display_none_dropped
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<p>visible</p><div style="display:none">secret hidden text</div><p>also visible</p>'
    _ = convert(html, None)

main()

```
