---
id: fixture_python_style_tags_only
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>"
    _ = convert(html, None)

main()

```
