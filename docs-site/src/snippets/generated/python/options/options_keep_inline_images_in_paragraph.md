---
id: fixture_python_options_keep_inline_images_in_paragraph
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Text <img src='icon.png' alt='icon'> more text</p>"
    options = ConversionOptions(keep_inline_images_in=["p"])
    _ = convert(html, options)

main()

```
