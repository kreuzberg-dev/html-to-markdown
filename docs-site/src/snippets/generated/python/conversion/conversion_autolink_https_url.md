---
id: fixture_python_conversion_autolink_https_url
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="https://example.com">https://example.com</a>'
    _ = convert(html, None)

main()

```
