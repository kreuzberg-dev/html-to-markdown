---
id: fixture_python_options_heading_style_underlined
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, HeadingStyle

def main() -> None:
    html = "<h1>Main Title</h1>"
    options = ConversionOptions(heading_style=HeadingStyle("Underlined"))
    _ = convert(html, options)

main()

```
