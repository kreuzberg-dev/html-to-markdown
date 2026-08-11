---
id: fixture_python_image_with_title
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<img src="chart.png" alt="Sales chart" title="Q3 Sales">'
    _ = convert(html, None)

main()

```
