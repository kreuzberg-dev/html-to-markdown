---
id: fixture_python_conversion_autolink_filename_not_autolinked
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="foobar.png">foobar.png</a>'
    _ = convert(html, None)

main()

```
