---
id: fixture_python_options_capture_svg_false
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Below SVG:</p><svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"/></svg>'
    options = ConversionOptions(capture_svg=False, extract_images=True)
    _ = convert(html, options)

main()

```
