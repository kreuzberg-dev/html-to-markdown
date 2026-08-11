---
id: fixture_python_conversion_autolink_relative_path_not_autolinked
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="/docs/intro.html">/docs/intro.html</a>'
    _ = convert(html, None)

main()

```
