---
id: fixture_python_options_highlight_bold
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, HighlightStyle

def main() -> None:
    html = "<p>Text with <mark>highlighted</mark> text.</p>"
    options = ConversionOptions(highlight_style=HighlightStyle("Bold"))
    _ = convert(html, options)

main()

```
