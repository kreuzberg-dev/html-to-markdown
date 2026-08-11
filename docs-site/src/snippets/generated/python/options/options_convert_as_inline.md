---
id: fixture_python_options_convert_as_inline
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>One</p><p>Two</p>"
    options = ConversionOptions(convert_as_inline=True)
    _ = convert(html, options)

main()

```
