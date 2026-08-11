---
id: fixture_python_options_strip_tags_div_span
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>"
    options = ConversionOptions(strip_tags=["div", "span"])
    _ = convert(html, options)

main()

```
