---
id: fixture_python_options_list_custom_bullets
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<ul><li>Item A</li><li>Item B</li></ul>"
    options = ConversionOptions(bullets="*")
    _ = convert(html, options)

main()

```
