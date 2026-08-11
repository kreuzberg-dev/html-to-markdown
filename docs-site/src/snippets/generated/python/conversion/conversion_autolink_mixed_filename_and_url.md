---
id: fixture_python_conversion_autolink_mixed_filename_and_url
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>'
    _ = convert(html, None)

main()

```
