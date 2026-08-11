---
id: fixture_python_options_newline_backslash
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, NewlineStyle

def main() -> None:
    html = "<p>Line one<br>Line two</p>"
    options = ConversionOptions(newline_style=NewlineStyle("Backslash"))
    _ = convert(html, options)

main()

```
